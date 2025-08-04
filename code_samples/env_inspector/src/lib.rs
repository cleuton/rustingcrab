//! Detect whether the current process runs in Kubernetes, a container, a VM, or on bare metal.

use std::{env, fs, path::Path};

/// Detected environment scope.
#[derive(Debug, PartialEq)]
pub enum EnvScope {
    /// Running inside a Kubernetes Pod.
    Kubernetes,
    /// Running inside a container; string is the container engine name.
    Container(String),
    /// Running inside a virtual machine; string is the hypervisor/vendor name.
    Vm(String),
    /// Running on bare metal (no container, no VM).
    BareMetal,
}

/// Returns true if we're in a Kubernetes Pod (via env‐var or serviceaccount token).
pub fn is_kubernetes() -> bool {
    env::var("KUBERNETES_SERVICE_HOST").is_ok()
        || Path::new("/var/run/secrets/kubernetes.io/serviceaccount/token").exists()
}

/// Try to identify container via files or cgroup.
/// 1) If `/.dockerenv` exists → docker
/// 2) If `/run/.containerenv` exists → podman
/// 3) Otherwise, look for known engines in `/proc/1/cgroup`
pub fn parse_container(contents: &str) -> Option<String> {
    if Path::new("/.dockerenv").exists() {
        return Some("docker".into());
    }
    if Path::new("/run/.containerenv").exists() {
        return Some("podman".into());
    }
    for engine in &["docker", "kubepods", "lxc", "rkt", "cri-o"] {
        if contents.contains(engine) {
            return Some(engine.to_string());
        }
    }
    None
}

/// Detect “hypervisor” flag in `/proc/cpuinfo`.
pub fn parse_vm_cpuinfo(contents: &str) -> bool {
    contents
        .lines()
        .filter(|l| l.starts_with("flags"))
        .any(|l| l.split_whitespace().any(|flag| flag == "hypervisor"))
}

/// Try to identify VM via DMI (`/sys/class/dmi/id/{product_name,sys_vendor}`).
pub fn parse_vm_dmi_field(value: &str) -> Option<String> {
    let v = value.trim().to_lowercase();
    for vendor in &[
        "kvm", "vmware", "virtualbox", "microsoft corporation",
        "xen", "amazon", "google",
    ] {
        if v.contains(vendor) {
            return Some(vendor.to_string());
        }
    }
    None
}

/// Perform detection in this order:
/// 0) Kubernetes  
/// 1) Container  
/// 2) VM (cpuinfo or DMI)  
/// 3) BareMetal
pub fn detect_scope() -> EnvScope {
    // 0) Kubernetes
    if is_kubernetes() {
        return EnvScope::Kubernetes;
    }

    // 1) Container
    if let Ok(cgroup) = fs::read_to_string("/proc/1/cgroup") {
        if let Some(engine) = parse_container(&cgroup) {
            return EnvScope::Container(engine);
        }
    }
    // Fallback file check alone
    if let Some(engine) = parse_container("") {
        return EnvScope::Container(engine);
    }

    // 2a) VM via cpuinfo
    if let Ok(cpuinfo) = fs::read_to_string("/proc/cpuinfo") {
        if parse_vm_cpuinfo(&cpuinfo) {
            return EnvScope::Vm("hypervisor".into());
        }
    }
    // 2b) VM via DMI
    for key in &["product_name", "sys_vendor"] {
        let path = format!("/sys/class/dmi/id/{}", key);
        if let Ok(val) = fs::read_to_string(&path) {
            if let Some(vendor) = parse_vm_dmi_field(&val) {
                return EnvScope::Vm(vendor);
            }
        }
    }

    // 3) Bare metal
    EnvScope::BareMetal
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_is_kubernetes_env_var() {
        env::remove_var("KUBERNETES_SERVICE_HOST");
        assert!(!is_kubernetes());
        env::set_var("KUBERNETES_SERVICE_HOST", "1");
        assert!(is_kubernetes());
        env::remove_var("KUBERNETES_SERVICE_HOST");
    }

    #[test]
    fn test_parse_container_cgroup() {
        assert_eq!(parse_container("12:devices:/docker/xyz"), Some("docker".into()));
        assert_eq!(parse_container("9:cpu:/kubepods/pod123"), Some("kubepods".into()));
        assert_eq!(parse_container("nothing here"), None);
    }

    #[test]
    fn test_parse_vm_cpuinfo() {
        let sample = "flags : fpu vme de pse hypervisor sse";
        assert!(parse_vm_cpuinfo(sample));
        assert!(!parse_vm_cpuinfo("flags : fpu sse"));
    }

    #[test]
    fn test_parse_vm_dmi_field() {
        assert_eq!(parse_vm_dmi_field("QEMU KVM"), Some("kvm".into()));
        assert_eq!(parse_vm_dmi_field("VMware, Inc."), Some("vmware".into()));
        assert_eq!(parse_vm_dmi_field("random"), None);
    }

    #[test]
    fn test_detect_scope_kubernetes() {
        env::set_var("KUBERNETES_SERVICE_HOST", "1");
        assert_eq!(detect_scope(), EnvScope::Kubernetes);
        env::remove_var("KUBERNETES_SERVICE_HOST");
    }

    #[test]
    fn test_detect_scope_bare_metal() {
        env::remove_var("KUBERNETES_SERVICE_HOST");
        assert_eq!(detect_scope(), EnvScope::BareMetal);
    }
}
