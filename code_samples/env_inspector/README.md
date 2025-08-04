<img src="../../rusting-crab-logo.png" height=300>


[**Cleuton Sampaio**](https://linkedin.com/in/cleutonsampaio)

[**See on GitHub**](https://github.com/cleuton/rustingcrab/blob/main/code_samples/env_inspector/README.md)


# env\_inspector

A minimal Rust crate to detect whether your application is running in a container, a virtual machine, or on bare metal (no container/VM).

## Features

* Detect common container environments (**Docker**, **Kubernetes**, **Podman**)
* Identify hypervisor flag in CPU info
* Inspect DMI fields to recognize VM vendors (KVM, VMware, VirtualBox, Hyper-V, Xen, AWS, GCP)
* No external dependencies or network calls

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
env_inspector = "0.1.0"
```

Then fetch the crate:

```bash
cargo build
```

## Usage

Call the `detect_scope` function to get an `EnvScope` enum:

```rust
use env_inspector::{detect_scope, EnvScope};

fn main() {
    match detect_scope() {
        EnvScope::Container(engine) => println!("Running in container: {}", engine),
        EnvScope::Vm(vendor)      => println!("Running in VM: {}", vendor),
        EnvScope::BareMetal      => println!("Running on bare metal"),
    }
}
```

## Running Tests

Execute the built‑in unit tests:

```bash
cargo test
```

## CLI Example

Optionally, include a small binary in `src/bin/inspect.rs`:

```rust
use env_inspector::detect_scope;

fn main() {
    println!("{:#?}", detect_scope());
}
```

Build and run it:

```bash
cargo run --bin inspect
```

The test code (`src/bin/inspect.rs`) should print: `BareMetal`.

## Running as a Docker container

Build the image and run a container: 

```
cargo build --release
docker build -t env-inspector .
docker run --rm env-inspector
```

You should see this message: 

```
Container(
    "docker",
)
```

If you run it with `podman` it can detect it: 

```
podman pull docker-daemon:env-inspector:latest
podman run --rm docker-daemon:env-inspector:latest

Container(
    "podman",
)

```

## Running in Kubernetes

1. **Build and push the Docker image**:

   ```bash
   # Build locally
   docker build -t your-registry/env-inspector:latest .
   # Push to your registry (Docker Hub, ECR, GCR, etc.)
   docker push your-registry/env-inspector:latest
   ```

2. **Create a Pod manifest** (`k8s/pod.yaml`):

   ```yaml
   apiVersion: v1
   kind: Pod
   metadata:
     name: env-inspector-test
   spec:
     containers:
       - name: inspector
         image: your-registry/env-inspector:latest
         command: ["inspect"]
     restartPolicy: Never
   ```

3. **Deploy to your cluster**:

   ```bash
   kubectl apply -f k8s/pod.yaml
   ```

4. **Check the logs**:

   ```bash
   kubectl logs pod/env-inspector-test
   ```

You should see output indicating `kubernetes`, confirming that the code correctly detected the environment inside the cluster.

## Running inside a VM Hypervisor

You need some `hypervisor` like `VirtualBox`. If you have `vagrant` then it is easy to run it with the supplied `Vagrantfile`: 

```
vagrant up

default: Vm(
default:     "hypervisor",
default: )

vagrant destroy -f
```

## Links

* Repository: [https://github.com/cleuton/rustingcrab/tree/main/code_samples/env_inspector](https://github.com/cleuton/rustingcrab/tree/main/code_samples/env_inspector)
* Issue Tracker: [https://github.com/cleuton/rustingcrab/issues](https://github.com/cleuton/rustingcrab/issues)
* License: MIT/Apache-2.0

