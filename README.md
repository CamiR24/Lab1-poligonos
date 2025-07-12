# Lab1-poligonos
# Rasterizador de Polígonos con Bresenham en Rust 🦀🎨

Este proyecto implementa un rasterizador simple de líneas y polígonos en Rust utilizando la biblioteca [raylib](https://www.raylib.com/). Se pueden dibujar y rellenar figuras poligonales complejas, aplicando el algoritmo de Bresenham y ray casting para detección de puntos internos.

---

## 🚀 Instrucciones para ejecutar

> ⚠️ **Importante**: Este repositorio ignora los archivos `Cargo.toml` y `Cargo.lock`, por lo que deberás crearlos manualmente.

---

### 1. Estructura del proyecto

├── src/
│   ├── main.rs
│   ├── framebuffer.rs
│   ├── line.rs
│   └── utils.rs
├── target/
├── Cargo.lock
├── Cargo.toml
├── .gitignore
├── README.md
└── out.png  (se genera al correr el programa)

### 2. Inicializa el proyecto de Rust

```bash
cargo init

#en Cargo.toml agrega la siguiente dependencia:
[dependencies]
raylib = "3.7"

#ejecuta el programa:
cargo run
```

### 3. Requisitos

Rust

libclang y dependencias de raylib (instalación depende del sistema operativo)

Un visor de imágenes como eog o tu visor predeterminado

### 3. Notas adicionales

El eje Y es invertido para simular coordenadas cartesianas tradicionales.

El algoritmo de relleno está optimizado usando un bounding box para recorrer solo los píxeles necesarios.

Puedes cambiar colores, puntos y más dentro de main.rs.

### 4. Imagen generada
![out](out.png)

