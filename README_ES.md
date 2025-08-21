# Cámara Web Fullscreen de Glastonbury

Una aplicación web simple y enfocada que proporciona una experiencia de visualización mejorada para la cámara web del Festival de Glastonbury.

🌐 **Sitio en vivo:** https://glastocam.foobarlol.lol

## 📋 Descripción

Esta aplicación web estática ofrece una vista dedicada y sin distracciones de la cámara web en vivo del Festival de Glastonbury. Está diseñada para entusiastas del festival que desean monitorear los terrenos del festival de forma remota con una experiencia de visualización optimizada.

### ✨ Características Principales

- **Vista en pantalla completa** - Experiencia de visualización inmersiva sin elementos de interfaz que distraigan
- **Actualización automática** - La imagen se actualiza cada 5 minutos (300 segundos) para mostrar contenido fresco
- **Cache-busting inteligente** - Implementa marcas de tiempo para asegurar que siempre se carguen imágenes actualizadas
- **Posicionamiento automático** - Ajusta automáticamente la vista para mostrar una perspectiva óptima de la imagen
- **Interfaz minimalista** - Diseño limpio y simple centrado únicamente en la experiencia de visualización

## 🛠 Stack Tecnológico

- **Frontend:** HTML5, CSS3, JavaScript puro (sin frameworks ni librerías)
- **Arquitectura:** Aplicación de una sola página (SPA) completamente del lado del cliente
- **Hosting:** Sitio web estático sin componentes del lado del servidor
- **Fuente de datos:** API externa de cámaras web Panomax (panodata2.panomax.com)

## 🚀 Instalación y Uso

### Requisitos Previos
- Navegador web moderno con soporte para JavaScript
- Conexión a internet para cargar las imágenes de la cámara web

### Instalación Local
1. Clona este repositorio:
   ```bash
   git clone [URL-del-repositorio]
   ```

2. Navega al directorio del proyecto:
   ```bash
   cd fullscreen-glasto-webcam
   ```

3. Abre `index.html` en tu navegador web preferido, o sirve los archivos usando un servidor web local:
   ```bash
   # Usando Python 3
   python3 -m http.server 8000
   
   # Usando Node.js (con http-server instalado globalmente)
   http-server
   
   # Usando PHP
   php -S localhost:8000
   ```

4. Accede a la aplicación en tu navegador en `http://localhost:8000`

### Uso
- Simplemente abre la página y disfruta de la vista en vivo de Glastonbury
- La imagen se actualizará automáticamente cada 5 minutos
- Usa las barras de desplazamiento para explorar diferentes partes de la imagen panorámica
- Para obtener la mejor experiencia, usa el modo de pantalla completa de tu navegador (F11 en la mayoría de navegadores)

## ⚙️ Cómo Funciona

La aplicación funciona mediante:

1. **Carga inicial:** Al cargar la página, se obtiene la imagen más reciente de la cámara web
2. **Cache-busting:** Cada solicitud incluye un timestamp único (`?ts=`) para evitar problemas de caché
3. **Auto-posicionamiento:** Una vez cargada la imagen, la vista se posiciona automáticamente en una ubicación óptima
4. **Actualización periódica:** Un temporizador actualiza la imagen cada 300 segundos (5 minutos)

### Detalles Técnicos
- **Fuente de la imagen:** `https://panodata2.panomax.com/cams/879/recent_full.jpg`
- **Intervalo de actualización:** 300,000 milisegundos (5 minutos)
- **Posicionamiento de scroll:** Calculado automáticamente para mostrar una vista balanceada

## 🎪 Sobre el Festival de Glastonbury

El Festival de Glastonbury es uno de los festivales de música y artes escénicas más icónicos del mundo, que se celebra en Worthy Farm, Somerset, Inglaterra. Esta aplicación permite a los fans y curiosos obtener una vista en tiempo real de los legendarios terrenos del festival.

## 📝 Nota del Desarrollador

> **Nota importante:** Este código está copiado directamente del marco de la página regular de Glastonbury, no hay hacking ni nada malo, solo una experiencia de visualización mejorada para el conocedor.

💚 **Ama la granja, no dejes rastro**

## 🤝 Contribuciones

Las contribuciones son bienvenidas. Por favor:

1. Haz fork del proyecto
2. Crea una rama para tu característica (`git checkout -b feature/nueva-caracteristica`)
3. Haz commit de tus cambios (`git commit -am 'Añadir nueva característica'`)
4. Push a la rama (`git push origin feature/nueva-caracteristica`)
5. Abre un Pull Request

## 📄 Licencia

Este proyecto es un trabajo derivado que utiliza contenido disponible públicamente del sitio web oficial de Glastonbury para proporcionar una mejor experiencia de usuario.

## 🔗 Enlaces Relacionados

- [Festival de Glastonbury - Sitio Oficial](https://www.glastonburyfestival.co.uk/)
- [Panomax - Proveedor de Cámaras Web](https://www.panomax.com/)

---

*Construido con ❤️ para la comunidad de Glastonbury*
