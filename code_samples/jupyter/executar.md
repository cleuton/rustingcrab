![](../../rusting-crab-logo.png)

# Rodando Rust no Jupyter

Cara, Rust é uma excelente opção para criar modelos de **Deep Learning** e fazer trabalhos de **Engenharia de dados**. 

Instale o **Docker**.

Suba um contêiner com essa imagem: 

```shell
docker run --rm \
  --platform linux/amd64 \
  -p 8888:8888 \
  -v "$(pwd)":/notebooks \
  cheperuiz/evcxr
```

Copie a URL que aparecerá na console... Algo assim: 

```shell
[I 14:51:55.936 NotebookApp] http://(893fa60207be or 127.0.0.1):8888/?token=9a485c86ddef2a33e902cb381b38e3dfe5e9c222756208fd
```

Edite para modificar o nome do computador ou deixar o localhost mesmo e cole em um navegador.

Pronto!