# Glastocam - Fullscreen Glastonbury Webcam Viewer

Een verbeterde kijkervaring voor de officiële Glastonbury Festival webcam in volledig scherm.

🌐 **Live Demo**: https://glastocam.foobarlol.lol

## 📖 Beschrijving

Glastocam biedt een schone, fullscreen weergave van de live webcam van het Glastonbury Festival. Deze eenvoudige webapplicatie extraheert de webcam-functionaliteit van de reguliere Glastonbury-pagina en presenteert deze zonder afleidingen voor een optimale kijkervaring.

## ✨ Functionaliteiten

- **Fullscreen weergave**: Maximaal schermgebruik voor de beste kijkervaring
- **Automatische verversing**: De webcam-afbeelding wordt elke 5 minuten (300 seconden) automatisch ververst
- **Cache-busting**: Gebruik van timestamps om ervoor te zorgen dat altijd de nieuwste afbeelding wordt geladen
- **Intelligente viewport positionering**: Automatische positionering naar het optimale deel van de afbeelding
- **Minimale interface**: Geen afleidende elementen, alleen de webcam

## 🛠 Technische Details

### Tech Stack
- **HTML5**: Basis structuur
- **CSS3**: Styling voor fullscreen weergave
- **Vanilla JavaScript**: Functionaliteit voor auto-refresh en positionering
- **Geen frameworks**: Pure web-technologieën voor maximale compatibiliteit en snelheid

### Architectuur
- **Statische website**: Client-side only applicatie
- **Single Page Application (SPA)**: Eén HTML-bestand met ingesloten CSS en JavaScript
- **Geen backend vereist**: Draait volledig in de browser
- **API Integratie**: Direct verbonden met de Panomax webcam API

## 📡 Webcam Integratie

De applicatie maakt gebruik van de **Panomax API** om live webcam-beelden van het Glastonbury Festival op te halen:

- **API Endpoint**: `https://panodata2.panomax.com/cams/879/recent_full.jpg`
- **Camera ID**: 879 (officiële Glastonbury Festival webcam)
- **Beeldformaat**: Full-resolution JPEG
- **Update frequentie**: Elke 5 minuten

### Cache-Busting Mechanisme
```javascript
img.src = "https://panodata2.panomax.com/cams/879/recent_full.jpg?ts=" + new Date().getTime();
```
Elke request bevat een unieke timestamp parameter om browser-caching te omzeilen en ervoor te zorgen dat altijd de nieuwste afbeelding wordt geladen.

## 🔄 Auto-Refresh Functionaliteit

De applicatie implementeert een intelligent refresh-systeem:

1. **Initiële load**: Webcam-afbeelding wordt geladen bij het opstarten
2. **Automatische positionering**: Na laden wordt de viewport automatisch gepositioneerd naar het meest interessante deel van de afbeelding
3. **Interval refresh**: Elke 5 minuten (300.000 milliseconden) wordt een nieuwe afbeelding opgehaald
4. **Viewport herpositionering**: Bij elke nieuwe afbeelding wordt de optimale weergavepositie herberekend

### Positionering Algoritme
```javascript
const scrollX = (document.body.scrollWidth - window.innerWidth) / 4;
const scrollY = document.body.scrollHeight;
window.scrollTo(scrollX, scrollY);
```
Dit algoritme positioneert de viewport ongeveer een kwart naar rechts en helemaal naar beneden, wat een optimaal zicht op het festivalterrein biedt.

## 🚀 Installatie & Deployment

### Lokaal Uitvoeren
1. **Clone de repository**:
   ```bash
   git clone [repository-url]
   cd glastocam
   ```

2. **Open in browser**:
   - Open `index.html` direct in je webbrowser
   - Of gebruik een lokale webserver:
   ```bash
   # Python 3
   python -m http.server 8000
   
   # Node.js (met http-server)
   npx http-server
   ```

3. **Ga naar**: `http://localhost:8000`

### Deployment
Als statische website kan Glastocam eenvoudig worden gehost op:
- **GitHub Pages**
- **Netlify**
- **Vercel**
- **Elke statische hosting provider**

Geen server-side configuratie vereist - upload gewoon de bestanden naar je hosting provider.

## 📁 Project Structuur

```
glastocam/
│
├── index.html          # Hoofdapplicatie (HTML, CSS, JavaScript)
├── README.md           # Deze documentatie
└── 2016-General-Site-Plan-v3.pdf  # Festival plattegrond (referentie)
```

## 🌱 Gebruik & Ethiek

**Belangrijke nota**: Deze applicatie gebruikt de officiële webcam-feed van het Glastonbury Festival. De code is gebaseerd op de implementatie van de officiële website, maar biedt een verbeterde gebruikerservaring zonder hacking of ongeautoriseerde toegang.

### Verantwoordelijk Gebruik
- Respecteer de bron van de webcam-feed
- Gebruik de applicatie niet voor commerciële doeleinden zonder toestemming
- Volg de algemene gebruiksvoorwaarden van het Glastonbury Festival

## 🤝 Bijdragen

Bijdragen zijn welkom! Voor verbeteringen:

1. Fork het project
2. Maak een feature branch (`git checkout -b feature/nieuwe-functie`)
3. Commit je wijzigingen (`git commit -am 'Voeg nieuwe functie toe'`)
4. Push naar de branch (`git push origin feature/nieuwe-functie`)
5. Open een Pull Request

## 🔗 Links & Credits

- **Live Site**: https://glastocam.foobarlol.lol
- **Glastonbury Festival**: https://glastonburyfestivals.co.uk/
- **Panomax Webcam Network**: https://www.panomax.com/

## 📄 Licentie

Dit project is een open-source implementatie voor educatieve en persoonlijke doeleinden. De webcam-feed blijft eigendom van Glastonbury Festival en Panomax.

---

**Love the farm, leave no trace** 💚

*Een project van webcam-enthousiastelingen voor de Glastonbury gemeenschap*
