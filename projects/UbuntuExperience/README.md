# Ubuntu Experience

A web-based interactive demo (PWA) of the Ubuntu Touch mobile operating system user interface.

## Live Demo

**Try it here**: https://kenneth-cho-infosec.github.io/UbuntuExperience/

## About

This is a **web-based simulation** of Ubuntu Touch - a Progressive Web App (PWA) that runs directly in your browser. It demonstrates the Ubuntu Touch user experience, featuring:

- Lock screen with time and notifications
- App launcher with grid of apps
- System settings
- Weather app
- Music player with library
- Terminal emulator with various commands
- File manager
- Gallery
- Browser
- Messaging and phone apps
- Task switcher
- Dark/Light mode toggle

**Note**: This is a UI demo/simulator only - it cannot run actual Ubuntu Touch apps or operating system.

## Installation

```bash
# Install dependencies
npm install

# Start development server
npm run dev

# Build for production
npm run build

# Preview production build
npm run preview
```

## Usage

1. Open the app in a browser
2. Unlock the screen (swipe up)
3. Browse through the app drawer
4. Try various apps and features
5. Toggle dark/light mode in settings
6. Use the size input to adjust demo dimensions

## Running as a PWA

To install as a PWA:
- Use the live demo link above
- Or serve the `dist` folder locally
- Open on a mobile device
- Add to home screen for fullscreen experience

## Technologies

- React 19
- Vite
- Lucide React (icons)
- CSS for styling

## License

MIT