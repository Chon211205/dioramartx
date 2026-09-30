# Galaxy Diorama

**Galaxy Diorama** es una experiencia interactiva desarrollada en Rust que presenta una colección de dioramas 3D distribuidos en tres galaxias. El proyecto utiliza un ray tracer ejecutado por CPU para representar planetas temáticos, materiales con texturas y propiedades ópticas, iluminación, sombras, reflexión, refracción y fondos espaciales procedurales.

Además de explorar cada escena, el jugador puede recorrer los mapas galácticos, rotar libremente los dioramas, acercar o alejar la cámara, recolectar destellos, desbloquear contenido y acceder a minijuegos integrados.

## Video de demostración

[![Ver demostración de Galaxy Diorama](https://img.youtube.com/vi/qTD66yRxyRo/maxresdefault.jpg)](https://www.youtube.com/watch?v=qTD66yRxyRo)

> Haz clic en la imagen para ver la demostración completa en YouTube.

## Características principales

- Tres galaxias explorables, cada una con ambientación, música y skybox propios.
- Veinte definiciones de planetas distribuidas entre las galaxias, con escenas como Forest Planet, Water Planet, Ice & Lava Planet, Cube Planet, Pyramid Planet, Chomp Planet, Kirby Planet y Mirror Planet.
- Renderizado 3D mediante ray tracing por CPU a una resolución interna de 1280 × 720.
- Cámara orbital con rotación libre, inercia y zoom.
- Materiales texturizados con mapas de color, normal, rugosidad, metalicidad y oclusión ambiental, según el material.
- Iluminación difusa y especular, sombras, emisión, reflexión y refracción recursivas.
- Skyboxes espaciales procedurales diferentes para cada galaxia.
- Diversas primitivas geométricas: esferas, cubos, cuboides, cilindros, conos, planos, hemisferios, elipsoides y toros.
- Aceleración de intersecciones mediante BVH y cajas delimitadoras AABB.
- Selector de galaxias, transiciones animadas, sistema de destellos y contenido desbloqueable.
- Música y efectos de sonido contextuales.
- Minijuegos integrados: **Tide Dash**, **Star Squadron** y **NOHC: Biohazard**.
- Compatibilidad con ventana redimensionable y pantalla completa.

## Criterios implementados

| Valor | Criterio | Evidencia en el proyecto |
|---:|---|---|
| 30 puntos | Complejidad de la escena | Tres galaxias, numerosos mundos compuestos por múltiples primitivas, transiciones, interacción, coleccionables, desbloqueables, audio, minijuegos y un motor de ray tracing modular. |
| 20 puntos | Atractivo visual | Planetas temáticos, paletas diferenciadas, texturas PBR, iluminación, efectos emisivos, fondos espaciales, animaciones y composiciones propias para cada mundo. |
| 20 puntos | Rotación y acercamiento/alejamiento | La cámara permite rotar los mapas y dioramas con el mouse, mantener inercia al soltar y controlar el zoom mediante la rueda. |
| 25 puntos | Cinco materiales diferentes | El proyecto supera los cinco materiales requeridos. Entre ellos hay madera, césped, roca, ladrillo, arena, hielo, lava y metal, con texturas propias y parámetros independientes de albedo, brillo especular, transparencia y reflectividad. |
| 10 puntos | Refracción | El ray tracer calcula rayos refractados para materiales transparentes. Se utiliza de manera contextual en superficies como hielo, agua/cristal y vidrio. |
| 5 puntos | Reflexión | Se mezclan rayos reflejados según la reflectividad de cada material; **Mirror Planet** demuestra este efecto de forma directa. |
| 20 puntos | Skybox | Cada una de las tres galaxias dispone de un skybox procedural propio, generado según la dirección del rayo para crear fondos espaciales continuos. |

**Total de criterios cubiertos: 130 puntos posibles**, incluyendo los apartados de valoración subjetiva.

## Galaxias y mundos

### Galaxia 1

- Forest Planet
- Water Planet
- Ice & Lava Planet
- Egg Planet
- Tree Planet
- Castle World

### Galaxia 2

- Cube Planet
- Water Race
- Pyramid Planet
- Industrial Planet
- Brick Planet
- Pokeball Planet
- Galaga Planet
- Castle Planet

### Galaxia 3

- Chomp Planet
- Brothers Planet
- Kirby Planet
- Mirror Planet
- Castle Planet
- Tower House, desbloqueable al alimentar la estrella hambrienta con destellos

## Controles

### Mapa galáctico

| Entrada | Acción |
|---|---|
| Clic izquierdo | Seleccionar un planeta o interactuar |
| Clic derecho + arrastrar | Rotar el mapa |
| `W`, `A`, `S`, `D` | Desplazarse por el mapa |
| Rueda del mouse | Acercar o alejar la cámara |
| `N` | Abrir o cerrar el selector de galaxias |
| `F11` | Alternar entre ventana y pantalla completa |

### Diorama enfocado

| Entrada | Acción |
|---|---|
| Clic izquierdo + arrastrar | Girar libremente el diorama |
| Soltar el clic | Conservar la inercia de la rotación |
| Rueda del mouse | Acercar o alejar la cámara |
| `P` | Iniciar el minijuego disponible en determinados planetas |
| `Backspace` | Regresar al mapa galáctico o salir de un minijuego |

El selector de galaxias también admite `A`/`D` o las flechas izquierda/derecha para cambiar la selección, `Enter` para confirmar y `Esc` para cerrarlo.

## Tecnologías

- [Rust](https://www.rust-lang.org/) — lenguaje y arquitectura general.
- [raylib-rs](https://github.com/deltaphc/raylib-rs) — ventana, entrada, audio y presentación del framebuffer.
- `nokhwa` — captura de cámara para el material especial de Mirror Planet.
- `rand` — generación de elementos y comportamientos variables.
- Ray tracer y sistema de materiales implementados dentro del proyecto.

## Requisitos

- Rust con Cargo (edición 2024; se recomienda una versión estable reciente).
- Una plataforma compatible con raylib.
- Dispositivo de audio para la música y los efectos.
- Cámara web opcional para la variante de material que admite captura de video.

## Ejecución

Clona el repositorio, entra en su directorio y ejecuta:

```bash
cargo run --release
```

El modo `--release` es recomendable porque el ray tracing se realiza en CPU. La primera compilación descargará y construirá las dependencias del proyecto.

Los minijuegos también están registrados como binarios independientes:

```bash
cargo run --release --bin tide_dash
cargo run --release --bin star_squadron
```

## Estructura del proyecto

```text
diorama/
├── assets/
│   ├── sounds/                 # Música y efectos de sonido
│   ├── sprites/                # Sprites usados por las escenas
│   └── textures/               # Texturas y mapas PBR por material
├── src/
│   ├── acceleration/           # BVH y cajas AABB
│   ├── core/                   # Vectores, rayos, cámara y framebuffer
│   ├── galaxies/               # Composición y registro de las 3 galaxias
│   ├── materials/              # Modelo y parámetros de materiales
│   ├── minigame/               # Tide Dash, Star Squadron y Biohazard
│   ├── objects/                # Primitivas e intersecciones geométricas
│   ├── renderer/               # Ray tracer, sombreado y skyboxes
│   ├── scene/                  # Escena, luces, planetas y estados
│   ├── textures/               # Carga y muestreo de texturas
│   ├── worlds/                 # Construcción de cada planeta/diorama
│   ├── main.rs                 # Bucle principal, interacción y transiciones
│   └── webcam.rs               # Captura opcional de cámara web
├── Cargo.toml                  # Configuración, dependencias y binarios
└── Cargo.lock                  # Versiones exactas de dependencias
```

## Arquitectura general

Cada mundo construye una colección de objetos geométricos y les asigna materiales. Las definiciones de planetas agrupan esos mundos dentro de cada galaxia. Durante el renderizado, la cámara genera un rayo por píxel; el BVH ayuda a localizar la intersección más cercana y el ray tracer calcula textura, iluminación, sombra y efectos ópticos. Si el rayo no golpea un objeto, se evalúa el skybox correspondiente a la galaxia actual.

El archivo `main.rs` coordina la navegación, la selección de planetas, la cámara, las transiciones, el audio, los destellos y el acceso a los minijuegos. Esta separación permite agregar nuevos mundos, formas o materiales sin concentrar toda la lógica dentro del bucle principal.

## Sistema de materiales

Cada material mantiene parámetros independientes para:

- color base y albedo;
- respuesta especular;
- transparencia;
- reflectividad;
- emisión e índice de refracción;
- textura de albedo;
- mapa normal;
- mapa de rugosidad;
- mapa de metalicidad;
- mapa de oclusión ambiental.

El renderer combina estas propiedades con la iluminación y con rayos secundarios. La reflexión toma la dirección especular del rayo incidente, mientras que la refracción aplica el índice óptico del material y contempla la reflexión interna total.

## Rendimiento

El renderizado es deliberadamente realizado por CPU. Para reducir su costo, la escena utiliza una jerarquía BVH, pruebas AABB y compilación optimizada en modo `release`. El framebuffer mantiene una resolución interna fija y se adapta a la ventana conservando la relación de aspecto.

