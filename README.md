# Tauri-Axum-Next Template

这是一个模板, 集合了Tauri, Axum, Next三个框架。

## Quaick Sstart

### Prerequist
- rust <= 1.79
- podman >= 5.1
- podman-compose

### Start up

设置环境
```bash

```

构建镜像
```bash
podman build . -t tauri-axum-next:latest --network host
```

启动容器
```bash
podman-compose up -d
```

检查日志
```bash
tail -f 
```

访问api
```bash
curl 
```
当获得以下输出时, 项目部署完成。
```json

```

测试
cargo watch -q -c -w src-axum/src -w configuration -x "run -p src-axum"
cargo watch -q -c -w src-axum/tests -w configuration -x "test -p src-axum test_config -- --nocapture"

### Contribute



为子项目添加依赖
```bash
pnpm add --filter="src-next" --save @tauri-apps/api 
```






