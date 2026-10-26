import { useEffect, useMemo, useRef, useState } from 'react'
import {
  AppWindow,
  ArrowLeft,
  BatteryFull,
  Bell,
  BookOpen,
  Bluetooth,
  Camera,
  Check,
  ChevronLeft,
  Clock,
  CloudSun,
  Download,
  Eye,
  FileText,
  Folder,
  Globe2,
  History,
  Image,
  Inbox,
  ListMusic,
  Lock,
  Mail,
  MapPin,
  Menu,
  MessageSquare,
  Mic,
  Moon,
  Music,
  Navigation,
  Pause,
  Phone,
  Play,
  Plus,
  Repeat,
  Search,
  Settings,
  Shield,
  ShoppingBag,
  Signal,
  Shuffle,
  SkipBack,
  SkipForward,
  Star,
  Sun,
  Terminal,
  Trash2,
  Upload,
  Users,
  Volume2,
  VolumeX,
  Wifi,
  X,
  Zap,
} from 'lucide-react'
import './App.css'

const assetPath = (path) => `${import.meta.env.BASE_URL}${path.replace(/^\//, '')}`

const appCatalog = [
  { id: 'browser', name: 'Morph Browser', short: 'Morph', icon: Globe2, color: 'cyan' },
  { id: 'gallery', name: 'Gallery', short: 'Gallery', icon: Image, color: 'sky' },
  { id: 'camera', name: 'Camera', short: 'Camera', icon: Camera, color: 'white' },
  { id: 'settings', name: 'System Settings', short: 'Settings', icon: Settings, color: 'silver' },
  { id: 'contacts', name: 'Contacts', short: 'Contacts', icon: Users, color: 'paper' },
  { id: 'messaging', name: 'Messaging', short: 'Messages', icon: MessageSquare, color: 'orange' },
  { id: 'phone', name: 'Phone', short: 'Phone', icon: Phone, color: 'green' },
  { id: 'store', name: 'OpenStore', short: 'Store', icon: ShoppingBag, color: 'black' },
  { id: 'terminal', name: 'Terminal', short: 'Terminal', icon: Terminal, color: 'charcoal' },
  { id: 'notes', name: 'Notes', short: 'Notes', icon: FileText, color: 'yellow' },
  { id: 'weather', name: 'Weather', short: 'Weather', icon: CloudSun, color: 'blue' },
  { id: 'music', name: 'Music', short: 'Music', icon: Music, color: 'pink' },
  { id: 'files', name: 'File Manager', short: 'Files', icon: Folder, color: 'gray' },
]

const launcherIds = ['browser', 'gallery', 'camera', 'store', 'settings', 'contacts', 'messaging', 'phone']

const storeApps = [
  {
    id: 'dekko',
    name: 'Dekko Mail',
    short: 'Dekko',
    icon: Mail,
    color: 'paper',
    category: 'Communication',
    rating: '4.7',
    size: '18 MB',
    summary: 'A focused mail client with folders, drafts, and a clean mobile reading pane.',
    permissions: ['Network', 'Contacts', 'Notifications'],
  },
  {
    id: 'podbird',
    name: 'Podbird',
    short: 'Podbird',
    icon: Mic,
    color: 'purple',
    category: 'Audio',
    rating: '4.5',
    size: '12 MB',
    summary: 'Subscribe to podcasts, queue episodes, and keep shows available offline.',
    permissions: ['Network', 'Storage', 'Audio'],
  },
  {
    id: 'teleports',
    name: 'Teleports',
    short: 'Teleports',
    icon: MessageSquare,
    color: 'cyan',
    category: 'Social',
    rating: '4.6',
    size: '24 MB',
    summary: 'A Telegram-style client shaped for Ubuntu Touch and small screens.',
    permissions: ['Network', 'Contacts', 'Notifications'],
  },
  {
    id: 'unav',
    name: 'uNav',
    short: 'uNav',
    icon: Navigation,
    color: 'green',
    category: 'Navigation',
    rating: '4.4',
    size: '9 MB',
    summary: 'Turn-by-turn navigation, places, and route previews for the demo phone.',
    permissions: ['Location', 'Network'],
  },
]



const defaultData = {
  demoWidth: 400,
  demoHeight: 180,
  darkMode: true,
  installedApps: [],
  status: {
    wifi: true,
    location: true,
    notifications: true,
    sound: true,
    bluetooth: true,
    batterySaver: false,
    network: '5G',
    battery: 84,
  },
  browser: {
    tabs: [{ id: 1, url: 'https://start.duckduckgo.com', title: 'DuckDuckGo', history: ['https://start.duckduckgo.com'], index: 0, private: false }],
    activeTab: 1,
    bookmarks: ['https://ubports.com', 'https://open-store.io'],
    history: ['https://start.duckduckgo.com'],
    downloads: [{ id: 1, name: 'ubuntu-touch-release-notes.pdf', size: '1.8 MB', date: 'Today' }],
  },
  gallery: [
    { id: 'ref-apps', src: assetPath('/references/apps-list.png'), label: 'App drawer reference', album: 'Screenshots', favorite: true, deleted: false, date: '2026-05-01' },
    { id: 'ref-device', src: assetPath('/references/pinephone-pro.jpg'), label: 'PinePhone Pro', album: 'Devices', favorite: false, deleted: false, date: '2026-04-28' },
    { id: 'ref-lock', src: assetPath('/references/lockscreen.jpg'), label: 'Lock screen reference', album: 'Screenshots', favorite: false, deleted: false, date: '2026-05-01' },
  ],
  contacts: [
    { id: 'ada', name: 'Ada Lovelace', phone: '+44 7700 900101', email: 'ada@example.test', favorite: true, group: 'Friends' },
    { id: 'grace', name: 'Grace Hopper', phone: '+1 202 555 0112', email: 'grace@example.test', favorite: true, group: 'Work' },
    { id: 'linus', name: 'Linus Torvalds', phone: '+358 40 555 0123', email: 'linus@example.test', favorite: false, group: 'Open Source' },
    { id: 'mary', name: 'Mary Jackson', phone: '+1 757 555 0148', email: 'mary@example.test', favorite: false, group: 'Work' },
  ],
  conversations: [
    {
      id: 'ada',
      contactId: 'ada',
      unread: 0,
      messages: [
        { from: 'them', text: 'Can I try Ubuntu Touch without flashing?', time: '14:02' },
        { from: 'me', text: 'Yes. This demo runs safely in your browser.', time: '14:04' },
      ],
    },
    {
      id: 'grace',
      contactId: 'grace',
      unread: 2,
      messages: [
        { from: 'them', text: 'The task switcher feels much smoother now.', time: '13:12' },
        { from: 'them', text: 'Try closing two background apps.', time: '13:13' },
      ],
    },
  ],
  callLog: [
    { id: 1, name: 'Grace Hopper', number: '+1 202 555 0112', type: 'incoming', time: 'Today 13:21' },
    { id: 2, name: 'Ada Lovelace', number: '+44 7700 900101', type: 'outgoing', time: 'Yesterday 18:04' },
  ],
  notes: [
    { id: 'daily', title: 'Daily setup', body: 'Ubuntu Experience demo notes\n- Open apps from the launcher\n- Install apps from OpenStore\n- Swipe right edge for background apps', tags: ['demo'], pinned: true, deleted: false, updated: 'Today' },
    { id: 'shopping', title: 'OpenStore ideas', body: 'Add realistic app detail pages, permissions, reviews, screenshots, and install progress.', tags: ['store'], pinned: false, deleted: false, updated: 'Yesterday' },
  ],
  files: [
    { id: 'documents', name: 'Documents', type: 'folder', path: '/', parent: '/', app: 'files' },
    { id: 'downloads', name: 'Downloads', type: 'folder', path: '/', parent: '/', app: 'files' },
    { id: 'pictures', name: 'Pictures', type: 'folder', path: '/', parent: '/', app: 'gallery' },
    { id: 'music-folder', name: 'Music', type: 'folder', path: '/', parent: '/', app: 'music' },
    { id: 'release', name: 'Ubuntu Touch release notes.txt', type: 'text', path: '/Documents', parent: 'documents', app: 'notes', content: 'Ubuntu Touch 24.04 demo release notes.' },
    { id: 'route', name: 'Saved route.gpx', type: 'map', path: '/Downloads', parent: 'downloads', app: 'unav', content: 'London to UBports office route preview.' },
  ],
  music: {
    playing: 'convergence',
    queue: ['convergence', 'lomiri', 'pocket'],
    tracks: [
      { id: 'convergence', title: 'Convergence', artist: 'Ubuntu Demo Band', album: 'Touch Mix', length: '3:42' },
      { id: 'lomiri', title: 'Lomiri Lights', artist: 'Dalton Street', album: 'Touch Mix', length: '4:05' },
      { id: 'pocket', title: 'Pocket Desktop', artist: 'Phablet Session', album: 'Daily Driver', length: '2:58' },
    ],
  },
  weather: {
    location: 'London',
    unit: 'C',
    condition: 'Partly cloudy',
    temp: 18,
    hourly: [18, 19, 19, 17, 15],
    daily: [
      ['Mon', 18, 'Clouds'],
      ['Tue', 21, 'Sunny'],
      ['Wed', 16, 'Rain'],
      ['Thu', 19, 'Clouds'],
      ['Fri', 22, 'Sunny'],
    ],
  },
  mail: [
    { id: 1, from: 'UBports News', subject: 'Ubuntu Touch 24.04 beta notes', body: 'Thanks for testing the new daily driver demo.', folder: 'Inbox', starred: true },
    { id: 2, from: 'OpenStore', subject: 'Your app finished installing', body: 'Dekko Mail is ready to open from the app drawer.', folder: 'Inbox', starred: false },
  ],
  podcasts: [
    { id: 'late-night-linux', show: 'Late Night Linux', episode: 'Ubuntu Touch on real hardware', progress: 42 },
    { id: 'linux-unplugged', show: 'Linux Unplugged', episode: 'The pocket desktop', progress: 12 },
  ],
  places: [
    { id: 'home', name: 'Home', detail: 'Saved place' },
    { id: 'ubports', name: 'UBports Foundation', detail: 'Route preview, 28 min' },
    { id: 'coffee', name: 'Coffee near me', detail: '5 min walk' },
  ],
}

function cloneDefaultData() {
  return JSON.parse(JSON.stringify(defaultData))
}

function usePersistentState(key, initialValue) {
  const [value, setValue] = useState(() => {
    try {
      const saved = window.localStorage.getItem(key)
      if (!saved) return initialValue
      return { ...initialValue, ...JSON.parse(saved) }
    } catch {
      return initialValue
    }
  })

  useEffect(() => {
    try {
      window.localStorage.setItem(key, JSON.stringify(value))
    } catch {
      // Storage can fail in private contexts; the demo still works in memory.
    }
  }, [key, value])

  return [value, setValue]
}

function App() {
  const [data, setData] = usePersistentState('ubuntu-experience-data-v3', cloneDefaultData())
  const [locked, setLocked] = useState(true)
  const [activeApp, setActiveApp] = useState(null)
  const [openApps, setOpenApps] = useState([])
  const [query, setQuery] = useState('')
  const [showDrawer, setShowDrawer] = useState(false)
  const [showTasks, setShowTasks] = useState(false)
  const [launcherVisible, setLauncherVisible] = useState(false)
  const [edgeDrag, setEdgeDrag] = useState(null)
  const [toasts, setToasts] = useState([])

  const isPWA = window.matchMedia('(display-mode: standalone)').matches || window.navigator.standalone === true

  const updateData = (updater) => setData((current) => (typeof updater === 'function' ? updater(current) : updater))
  const notify = (text) => {
    const id = Date.now()
    setToasts((current) => [...current, { id, text }])
    window.setTimeout(() => setToasts((current) => current.filter((toast) => toast.id !== id)), 2600)
  }

  const allApps = useMemo(() => {
    const installedStoreApps = storeApps.filter((app) => data.installedApps.includes(app.id))
    return [...appCatalog, ...installedStoreApps]
  }, [data.installedApps])

  const filteredApps = useMemo(() => {
    return allApps.filter((app) => app.name.toLowerCase().includes(query.toLowerCase()))
  }, [allApps, query])

  const openApp = (id) => {
    setOpenApps((current) => (current.includes(id) ? current : [...current, id]))
    setActiveApp(id)
    setShowDrawer(false)
    setShowTasks(false)
    setLauncherVisible(false)
  }

  const goHome = () => {
    setActiveApp(null)
    setShowDrawer(false)
    setShowTasks(false)
    setLauncherVisible(false)
  }

  const openAppDrawer = () => {
    setActiveApp(null)
    setShowDrawer((current) => !current)
    setShowTasks(false)
    setLauncherVisible(false)
  }

  const installApp = (id) => {
    updateData((current) => ({
      ...current,
      installedApps: current.installedApps.includes(id) ? current.installedApps : [...current.installedApps, id],
    }))
    notify(`${storeApps.find((app) => app.id === id)?.name || 'App'} installed`)
  }

  const removeApp = (id) => {
    updateData((current) => ({ ...current, installedApps: current.installedApps.filter((appId) => appId !== id) }))
    setOpenApps((current) => current.filter((appId) => appId !== id))
    if (activeApp === id) goHome()
    notify('App removed')
  }

  const resetDemo = () => {
    updateData(cloneDefaultData())
    setOpenApps([])
    goHome()
    notify('Demo data reset')
  }

  const closeBackgroundApp = (id) => {
    setOpenApps((current) => {
      const next = current.filter((appId) => appId !== id)
      if (next.length === 0) {
        setShowTasks(false)
        setActiveApp(null)
        setLauncherVisible(false)
      } else if (activeApp === id) {
        setActiveApp(next[next.length - 1])
      }
      return next
    })
  }

  const startEdgeGesture = (event) => {
    if (!activeApp || showTasks) return
    const bounds = event.currentTarget.getBoundingClientRect()
    const localX = event.clientX - bounds.left
    if (launcherVisible && localX > 76) {
      setLauncherVisible(false)
      return
    }
    if (localX < 28) {
      setEdgeDrag({ side: 'left', x: event.clientX })
      event.currentTarget.setPointerCapture(event.pointerId)
    } else if (localX > bounds.width - 28) {
      setEdgeDrag({ side: 'right', x: event.clientX })
      event.currentTarget.setPointerCapture(event.pointerId)
    }
  }

  const startForcedEdgeGesture = (side, event) => {
    if (!activeApp || showTasks) return
    event.stopPropagation()
    setEdgeDrag({ side, x: event.clientX })
    event.currentTarget.setPointerCapture?.(event.pointerId)
  }

  const moveEdgeGesture = (event) => {
    if (!edgeDrag) return
    const delta = event.clientX - edgeDrag.x
    if (edgeDrag.side === 'left' && delta > 48) {
      setLauncherVisible(true)
      setEdgeDrag(null)
    }
    if (edgeDrag.side === 'right' && delta < -48) {
      setShowTasks(true)
      setLauncherVisible(false)
      setEdgeDrag(null)
    }
  }

  const finishEdgeGesture = () => setEdgeDrag(null)

  useEffect(() => {
    const resetHorizontalScroll = () => {
      window.scrollTo({ left: 0, top: window.scrollY })
      document.documentElement.scrollLeft = 0
      document.body.scrollLeft = 0
      document.querySelectorAll('.phone-shell').forEach((element) => {
        element.style.transform = 'none'
      })
      document.querySelectorAll('*').forEach((element) => {
        element.scrollLeft = 0
      })
    }
    resetHorizontalScroll()
    const frame = window.requestAnimationFrame(resetHorizontalScroll)
    const timers = [
      window.setTimeout(resetHorizontalScroll, 0),
      window.setTimeout(resetHorizontalScroll, 120),
      window.setTimeout(resetHorizontalScroll, 300),
    ]
    return () => {
      window.cancelAnimationFrame(frame)
      timers.forEach((timer) => window.clearTimeout(timer))
    }
  }, [activeApp, showDrawer, showTasks, launcherVisible])

  const active = allApps.find((app) => app.id === activeApp)
  const backgroundApps = openApps.map((id) => allApps.find((app) => app.id === id)).filter(Boolean)
  const shellMode = showTasks ? 'tasks' : active ? 'app' : showDrawer ? 'drawer' : 'home'
  const launcherState = !active ? 'docked' : launcherVisible ? 'revealed' : 'minimized'

  return (
    <main className={`demo-page ${data.darkMode ? 'dark-mode' : 'light-mode'} ${isPWA ? 'pwa-mode' : ''}`}>
      {!isPWA && <DemoSizeControls demoWidth={data.demoWidth} setDemoWidth={(w) => updateData((current) => ({ ...current, demoWidth: w }))} demoHeight={data.demoHeight} setDemoHeight={(h) => updateData((current) => ({ ...current, demoHeight: h }))} />}
      <section className={`phone-shell mode-${shellMode} launcher-${launcherState} ${active ? 'app-open' : ''} ${edgeDrag ? 'edge-swiping' : ''} ${data.darkMode ? 'dark-mode' : 'light-mode'} ${isPWA ? 'pwa-fullscreen' : ''}`} style={{ '--shell-width': `${data.demoWidth}px`, '--shell-height': `${data.demoHeight}px` }} aria-label="Ubuntu Experience interactive demo">
        {locked ? (
          <LockScreen
            status={data.status}
            onUnlock={() => {
              setLocked(false)
              goHome()
            }}
          />
        ) : (
          <>
            <StatusBar status={data.status} setStatus={(status) => updateData((current) => ({ ...current, status }))} />
            <Launcher
              openApp={openApp}
              goHome={openAppDrawer}
              activeApp={activeApp}
              showDrawer={showDrawer}
              state={showTasks ? 'hidden' : launcherState}
            />
            <div
              className={`workspace shell-page shell-page-${shellMode} ${active ? 'app-workspace' : ''}`}
              onPointerDown={startEdgeGesture}
              onPointerMove={moveEdgeGesture}
              onPointerUp={finishEdgeGesture}
              onPointerCancel={finishEdgeGesture}
            >
              <Wallpaper />
              {active && !launcherVisible && !showTasks && (
                <>
                  <div className="edge-gesture-zone left" onPointerDown={(event) => startForcedEdgeGesture('left', event)} aria-hidden="true" />
                  <div className="edge-gesture-zone right" onPointerDown={(event) => startForcedEdgeGesture('right', event)} aria-hidden="true" />
                </>
              )}
              {showTasks ? (
                <TaskSwitcher apps={backgroundApps} activeApp={activeApp} openApp={openApp} closeApp={closeBackgroundApp} goHome={goHome} />
              ) : active ? (
                <AppSurface
                  app={active}
                  darkMode={data.darkMode}
                  openApp={openApp}
                  data={data}
                  updateData={updateData}
                  installApp={installApp}
                  removeApp={removeApp}
                  resetDemo={resetDemo}
                  notify={notify}
                />
              ) : (
                <HomeScreen apps={filteredApps} query={query} setQuery={setQuery} showDrawer={showDrawer} openApp={openApp} />
              )}
            </div>
            <ToastStack toasts={toasts} />
          </>
        )}
      </section>
    </main>
  )
}

function DemoSizeControls({ demoWidth, setDemoWidth, demoHeight, setDemoHeight }) {
  const ratio = `${demoWidth}:${demoHeight}`

  const handleRatioChange = (e) => {
    const match = e.target.value.match(/^(\d+):(\d+)$/)
    if (match) {
      const w = Number(match[1])
      const h = Number(match[2])
      if (w > 0 && h > 0) {
        const newWidth = Math.round((demoHeight / h) * w)
        setDemoWidth(newWidth)
      }
    }
  }

  const handleWidthChange = (e) => {
    const w = Number(e.target.value)
    if (w > 0) setDemoWidth(w)
  }

  const handleHeightChange = (e) => {
    const h = Number(e.target.value)
    if (h > 0) setDemoHeight(h)
  }

  return (
    <div className="demo-controls" aria-label="Demo screen size">
      <span>Size</span>
      <div className="size-inputs">
        <input type="text" value={ratio} onChange={handleRatioChange} placeholder="20:9" pattern="\d+:\d+" />
        <input type="number" value={demoWidth} onChange={handleWidthChange} placeholder="W" min={200} max={2000} />
        <span>x</span>
        <input type="number" value={demoHeight} onChange={handleHeightChange} placeholder="H" min={100} max={2000} />
      </div>
    </div>
  )
}

function ToastStack({ toasts }) {
  return (
    <div className="toast-stack" aria-live="polite">
      {toasts.map((toast) => (
        <div key={toast.id} className="toast">
          <Check size={16} />
          {toast.text}
        </div>
      ))}
    </div>
  )
}

function LockScreen({ status, onUnlock }) {
  const [time, setTime] = useState(() => new Date())
  const [dragStart, setDragStart] = useState(null)
  const [dragX, setDragX] = useState(0)

  useEffect(() => {
    const timer = window.setInterval(() => setTime(new Date()), 1000)
    return () => window.clearInterval(timer)
  }, [])

  const startSwipe = (event) => {
    setDragStart(event.clientX)
    event.currentTarget.setPointerCapture(event.pointerId)
  }

  const moveSwipe = (event) => {
    if (dragStart === null) return
    const nextDragX = Math.min(Math.max(event.clientX - dragStart, -150), 150)
    setDragX(nextDragX)
    if (Math.abs(nextDragX) > 86) onUnlock()
  }

  const finishSwipe = (event) => {
    const finalDragX = dragStart === null ? dragX : event.clientX - dragStart
    if (Math.abs(finalDragX) > 86) {
      onUnlock()
      return
    }
    setDragStart(null)
    setDragX(0)
  }

  const unlockProgress = Math.min(Math.abs(dragX) / 86, 1)

  return (
    <section className="lock-screen" aria-label="Lock screen" onPointerDown={startSwipe} onPointerMove={moveSwipe} onPointerUp={finishSwipe} onPointerCancel={finishSwipe}>
      <div className="lock-status">
        <Mail size={17} />
        <MapPin size={17} />
        <Bluetooth size={16} className={status.bluetooth ? '' : 'dimmed'} />
        <Wifi size={18} className={status.wifi ? '' : 'dimmed'} />
        {status.sound ? <Volume2 size={17} /> : <VolumeX size={17} />}
        <BatteryFull size={22} />
        <time>{time.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit', hour12: false })}</time>
      </div>
      <div className="lock-time">
        <time>{time.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit', hour12: false })}</time>
        <span>{time.toLocaleDateString([], { weekday: 'long', day: 'numeric', month: 'long', year: 'numeric' })}</span>
      </div>
      <div className="photo-ring" aria-label="Daily photo activity">
        <div className="ring-dots">
          {Array.from({ length: 32 }).map((_, index) => (
            <span key={index} style={{ transform: `rotate(${index * 11.25}deg) translateY(-106px)` }} />
          ))}
        </div>
        <div className="ring-drop" />
        <strong>No photos taken today</strong>
      </div>
      <div className="unlock-track" style={{ '--unlock-x': `${dragX}px`, '--unlock-progress': unlockProgress }} role="slider" aria-label="Swipe left or right to unlock" aria-valuemin="0" aria-valuemax="100" aria-valuenow={Math.round(unlockProgress * 100)}>
        <span className="unlock-chevrons">&lsaquo;</span>
        <span className="unlock-label">Unlock</span>
        <span className="unlock-chevrons">&rsaquo;</span>
      </div>
    </section>
  )
}

function StatusBar({ status, setStatus }) {
  const [time, setTime] = useState(() => new Date())
  const [open, setOpen] = useState(false)
  const [dragStart, setDragStart] = useState(null)
  const [dragY, setDragY] = useState(0)
  const [activeTab, setActiveTab] = useState('network')
  const [brightness, setBrightness] = useState(72)
  const [blueLightFilter, setBlueLightFilter] = useState(false)

  useEffect(() => {
    const timer = window.setInterval(() => setTime(new Date()), 1000)
    return () => window.clearInterval(timer)
  }, [])

  useEffect(() => {
    const shell = document.querySelector('.phone-shell')
    if (shell) {
      const brightnessValue = brightness / 100
      shell.style.filter = `brightness(${brightnessValue})`
    }
  }, [brightness])

  const update = (patch) => setStatus({ ...status, ...patch })
  const toggle = (key) => update({ [key]: !status[key] })
  const startSwipe = (event) => {
    setDragStart(event.clientY)
    event.currentTarget.setPointerCapture(event.pointerId)
  }

  const moveSwipe = (event) => {
    if (dragStart === null) return
    const delta = Math.min(event.clientY - dragStart, 0)
    setDragY(Math.max(delta, -180))
    if (dragY < -60) setOpen(true)
  }

  const finishSwipe = () => {
    if (dragY < -80) {
      setOpen(true)
    } else {
      setOpen(false)
    }
    setDragStart(null)
    setDragY(0)
  }

  const handleWheel = (event) => {
    if (event.deltaY > 0 && !open) {
      setOpen(true)
    } else if (event.deltaY < 0 && open) {
      setOpen(false)
    }
  }

  const closePanel = () => setOpen(false)

  const tabs = [
    { id: 'network', label: 'Network', icon: Signal },
    { id: 'sound', label: 'Sound', icon: Volume2 },
    { id: 'battery', label: 'Battery', icon: BatteryFull },
    { id: 'datetime', label: 'Time & Date', icon: Clock },
    { id: 'system', label: 'System', icon: Settings },
  ]

  const wifiNetworks = [
    { name: 'StarLink_5G', signal: 4, locked: true, active: true },
    { name: 'CosmosNet', signal: 3, locked: false, active: false },
    { name: 'Nebula_Free', signal: 2, locked: false, active: false },
  ]

  return (
    <>
      <header className="status-bar" onPointerDown={startSwipe} onPointerMove={moveSwipe} onPointerUp={finishSwipe} onPointerCancel={finishSwipe} onWheel={handleWheel}>
        <button className="status-left" onClick={() => setOpen(!open)} aria-expanded={open} aria-label="Indicators" title="Indicators">
          <Mail size={16} className={status.notifications ? '' : 'dimmed'} />
          <MapPin size={16} className={status.location ? '' : 'dimmed'} />
          <Bluetooth size={16} className={status.bluetooth ? '' : 'dimmed'} />
        </button>
        <button className="status-right" onClick={() => setOpen(!open)} aria-expanded={open} aria-label="Quick settings" title="Quick settings">
          <Signal size={17} />
          <span className="network">{status.network}</span>
          <Wifi size={17} className={status.wifi ? '' : 'dimmed'} />
          {status.sound ? <Volume2 size={17} /> : <VolumeX size={17} />}
          <BatteryFull size={20} />
          <span className="battery-percent">{status.battery}%</span>
          <time>{time.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}</time>
        </button>
      </header>
      {open && (
        <section className="quick-settings-panel" onWheel={handleWheel}>
          <header className="quick-settings-header">
            <button className="quick-settings-close" onClick={closePanel}>
              <ChevronLeft size={24} />
            </button>
            <span>Quick Settings</span>
          </header>
          <nav className="quick-tabs">
            {tabs.map((tab) => {
              const Icon = tab.icon
              return (
                <button key={tab.id} className={`quick-tab ${activeTab === tab.id ? 'active' : ''}`} onClick={() => setActiveTab(tab.id)}>
                  <Icon size={20} />
                  <span>{tab.label}</span>
                </button>
              )
            })}
          </nav>
          <div className="quick-panel-content">
            {activeTab === 'network' && (
              <div className="quick-panel-section">
                <div className="quick-toggle-row">
                  <div className="quick-toggle-info">
                    <Wifi size={20} />
                    <span>Wi-Fi</span>
                  </div>
                  <button className={`quick-toggle-switch ${status.wifi ? 'on' : ''}`} onClick={() => toggle('wifi')}>
                    <span className="toggle-thumb" />
                  </button>
                </div>
                <div className="wifi-list">
                  {wifiNetworks.map((network) => (
                    <div key={network.name} className={`wifi-item ${network.active ? 'active' : ''}`}>
                      <Wifi size={18} className={network.active ? 'active' : ''} />
                      <span className="wifi-name">{network.name}</span>
                      {network.locked && <Lock size={16} />}
                    </div>
                  ))}
                </div>
                <div className="quick-toggle-row">
                  <div className="quick-toggle-info">
                    <Bluetooth size={20} />
                    <span>Bluetooth</span>
                  </div>
                  <button className={`quick-toggle-switch ${status.bluetooth ? 'on' : ''}`} onClick={() => toggle('bluetooth')}>
                    <span className="toggle-thumb" />
                  </button>
                </div>
                <div className="quick-toggle-row">
                  <div className="quick-toggle-info">
                    <MapPin size={20} />
                    <span>Location</span>
                  </div>
                  <button className={`quick-toggle-switch ${status.location ? 'on' : ''}`} onClick={() => toggle('location')}>
                    <span className="toggle-thumb" />
                  </button>
                </div>
              </div>
            )}
            {activeTab === 'sound' && (
              <div className="quick-panel-section">
                <div className="quick-toggle-row">
                  <div className="quick-toggle-info">
                    <Volume2 size={20} />
                    <span>Sound</span>
                  </div>
                  <button className={`quick-toggle-switch ${status.sound ? 'on' : ''}`} onClick={() => toggle('sound')}>
                    <span className="toggle-thumb" />
                  </button>
                </div>
                <div className="quick-toggle-row">
                  <div className="volume-slider-container">
                    <VolumeX size={16} />
                    <input 
                      type="range" 
                      min="0" 
                      max="100" 
                      value={status.sound ? 72 : 0} 
                      className="volume-slider" 
                      onChange={(event) => update({ sound: Number(event.target.value) })}
                    />
                    <Volume2 size={16} />
                  </div>
                </div>
                <div className="quick-toggle-row">
                  <div className="quick-toggle-info">
                    <Bell size={20} />
                    <span>Notifications</span>
                  </div>
                  <button className={`quick-toggle-switch ${status.notifications ? 'on' : ''}`} onClick={() => toggle('notifications')}>
                    <span className="toggle-thumb" />
                  </button>
                </div>
              </div>
            )}
            {activeTab === 'battery' && (
              <div className="quick-panel-section">
                <div className="battery-display">
                  <span className="battery-percent-large">{status.battery}%</span>
                  <span className="battery-label">Charge level</span>
                </div>
                <div className="toggle-slider brightness-slider">
                  <Sun size={18} />
                  <input type="range" min="0" max="100" value={brightness} onChange={(event) => setBrightness(Number(event.target.value))} />
                  <Sun size={18} />
                </div>
                <div className="quick-toggle-row">
                  <div className="quick-toggle-info">
                    <Zap size={20} />
                    <span>Battery saver</span>
                  </div>
                  <button className={`quick-toggle-switch ${status.batterySaver ? 'on' : ''}`} onClick={() => toggle('batterySaver')}>
                    <span className="toggle-thumb" />
                  </button>
                </div>
                <div className="quick-toggle-row">
                  <div className="quick-toggle-info">
                    <Eye size={20} />
                    <span>Blue light filter</span>
                  </div>
                  <button className={`quick-toggle-switch ${blueLightFilter ? 'on' : ''}`} onClick={() => {
                    setBlueLightFilter(!blueLightFilter)
                    const shell = document.querySelector('.phone-shell')
                    if (shell) {
                      if (blueLightFilter) {
                        shell.style.filter = `brightness(${brightness / 100})`
                      } else {
                        shell.style.filter = `brightness(${brightness / 100}) sepia(0.6) hue-rotate(-30deg) saturate(1.2)`
                      }
                    }
                  }}>
                    <span className="toggle-thumb" />
                  </button>
                </div>
              </div>
            )}
            {activeTab === 'datetime' && (
              <div className="quick-panel-section">
                <div className="datetime-display">
                  <span className="time-large">{time.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}</span>
                  <span className="date-large">{time.toLocaleDateString([], { weekday: 'long', month: 'long', day: 'numeric' })}</span>
                </div>
                <div className="quick-toggle-row">
                  <div className="quick-toggle-info">
                    <Clock size={20} />
                    <span>Auto-sync time</span>
                  </div>
                  <button className="quick-toggle-switch on">
                    <span className="toggle-thumb" />
                  </button>
                </div>
                <div className="quick-toggle-row">
                  <div className="quick-toggle-info">
                    <Globe2 size={20} />
                    <span>Time zone</span>
                    <small>UTC+01:00 London</small>
                  </div>
                </div>
              </div>
            )}
            {activeTab === 'system' && (
              <div className="quick-panel-section">
                <div className="quick-toggle-row">
                  <div className="quick-toggle-info">
                    <Star size={20} />
                    <span>Flight mode</span>
                  </div>
                  <button className="quick-toggle-switch off">
                    <span className="toggle-thumb" />
                  </button>
                </div>
                <div className="quick-toggle-row">
                  <div className="quick-toggle-info">
                    < Wifi size={20} />
                    <span>Hotspot</span>
                  </div>
                  <button className="quick-toggle-switch off">
                    <span className="toggle-thumb" />
                  </button>
                </div>
                <div className="system-info">
                  <span>Ubuntu Touch Demo</span>
                  <small>Version 24.04</small>
                </div>
              </div>
            )}
          </div>
        </section>
      )}
    </>
  )
}

function Launcher({ openApp, goHome, activeApp, showDrawer, state = 'docked' }) {
  const launcherApps = launcherIds.map((id) => appCatalog.find((app) => app.id === id))
  const visible = state !== 'hidden' && state !== 'minimized'
  return (
    <nav className={`launcher launcher-${state} ${visible ? 'visible' : 'hidden'}`} aria-label="Ubuntu launcher app bar" aria-hidden={!visible}>
      <div className="launcher-stack">
        {launcherApps.map((app) => (
          <button className={`launcher-button ${activeApp === app.id ? 'active' : ''}`} key={app.id} onPointerDown={(event) => event.stopPropagation()} onPointerUp={(event) => {
            event.stopPropagation()
            openApp(app.id)
          }} onClick={() => openApp(app.id)} title={app.name}>
            <AppIcon app={app} compact />
          </button>
        ))}
      </div>
      <button className={`ubuntu-home ${showDrawer ? 'drawer-open' : ''}`} onClick={goHome} aria-label={showDrawer ? 'Close applications' : 'Open applications'} title={showDrawer ? 'Close applications' : 'Open applications'}>
        <img src={assetPath('/ubuntu-cof.svg')} alt="Ubuntu home" />
      </button>
    </nav>
  )
}

function HomeScreen({ apps, query, setQuery, showDrawer, openApp }) {
  return (
    <div className={`home-screen ${showDrawer ? 'drawer-open' : 'home-idle'}`}>
      {showDrawer && (
        <section className="app-drawer" aria-label="Applications">
          <label className="search-box">
            <Search size={22} />
            <input value={query} onChange={(event) => setQuery(event.target.value)} placeholder="Search..." aria-label="Search applications" />
          </label>
          <div className="apps-grid">
            {apps.map((app) => (
              <button className="app-tile" key={app.id} onClick={() => openApp(app.id)}>
                <AppIcon app={app} />
                <span>{app.name}</span>
              </button>
            ))}
          </div>
        </section>
      )}
    </div>
  )
}

function Wallpaper() {
  return (
    <div className="wallpaper" aria-hidden="true">
      <div className="line-emblem" />
    </div>
  )
}

function AppIcon({ app, compact = false }) {
  const Icon = app.icon
  return (
    <span className={`app-icon ${app.color} ${compact ? 'compact' : ''}`}>
      <Icon size={compact ? 26 : 38} strokeWidth={compact ? 2.4 : 2.1} />
    </span>
  )
}

function AppSurface({ app, openApp, data, updateData, installApp, removeApp, resetDemo, notify, darkMode }) {
  return (
    <section className={`app-surface app-page ${darkMode ? 'dark-mode' : 'light-mode'}`} aria-label={`${app.name} app page`}>
      <div className={`app-content ${darkMode ? 'dark-mode' : 'light-mode'}`}>
        <AppContent id={app.id} app={app} openApp={openApp} data={data} updateData={updateData} installApp={installApp} removeApp={removeApp} resetDemo={resetDemo} notify={notify} />
      </div>
    </section>
  )
}

function AppContent({ id, app, openApp, data, updateData, installApp, removeApp, resetDemo, notify }) {
  const common = { data, updateData, openApp, notify, darkMode: data.darkMode }
  if (id === 'browser') return <BrowserApp {...common} />
  if (id === 'gallery') return <GalleryApp {...common} />
  if (id === 'camera') return <CameraApp {...common} />
  if (id === 'messaging') return <MessagingApp {...common} />
  if (id === 'phone') return <PhoneApp {...common} />
  if (id === 'settings') return <SettingsApp {...common} resetDemo={resetDemo} />
  if (id === 'terminal') return <TerminalApp {...common} installApp={installApp} />
  if (id === 'notes') return <NotesApp {...common} />
  if (id === 'weather') return <WeatherApp {...common} />
  if (id === 'store') return <StoreApp openApp={openApp} installedApps={data.installedApps} installApp={installApp} removeApp={removeApp} />
  if (id === 'music') return <MusicApp {...common} />
  if (id === 'music') return <MusicApp {...common} />
  if (id === 'contacts') return <ContactsApp {...common} />
  if (id === 'files') return <FilesApp {...common} />
  if (id === 'dekko') return <DekkoApp {...common} removeApp={removeApp} app={app} />
  if (id === 'podbird') return <PodbirdApp {...common} removeApp={removeApp} app={app} />
  if (id === 'teleports') return <TeleportsApp {...common} removeApp={removeApp} app={app} />
  if (id === 'unav') return <UNavApp {...common} removeApp={removeApp} app={app} />
  return <GenericApp app={app} removeApp={removeApp} />
}

function TaskSwitcher({ apps, activeApp, openApp, closeApp, goHome }) {
  const [dragging, setDragging] = useState(null)
  const [closingId, setClosingId] = useState(null)
  const [suppressClickId, setSuppressClickId] = useState(null)

  useEffect(() => {
    if (apps.length === 0) {
      const timer = window.setTimeout(goHome, 120)
      return () => window.clearTimeout(timer)
    }
  }, [apps.length, goHome])

  const beginDrag = (event, id) => {
    event.stopPropagation()
    event.currentTarget.setPointerCapture?.(event.pointerId)
    setDragging({ id, startY: event.clientY, offsetY: 0, moved: false })
  }

  const moveDrag = (event) => {
    if (!dragging || closingId) return
    event.stopPropagation()
    const offsetY = Math.min(Math.max(event.clientY - dragging.startY, -190), 42)
    setDragging({ ...dragging, offsetY, moved: Math.abs(offsetY) > 8 })
  }

  const closeWithAnimation = (id) => {
    setClosingId(id)
    setSuppressClickId(id)
    window.setTimeout(() => {
      closeApp(id)
      setClosingId(null)
      setSuppressClickId(null)
    }, 170)
  }

  const finishDrag = (event, appId) => {
    if (!dragging || dragging.id !== appId) return
    event.stopPropagation()
    event.currentTarget.releasePointerCapture?.(event.pointerId)
    const shouldClose = dragging.offsetY < -70
    if (dragging.moved) {
      setSuppressClickId(appId)
      window.setTimeout(() => setSuppressClickId(null), 220)
    }
    setDragging(null)
    if (shouldClose) closeWithAnimation(appId)
  }

  const selectApp = (appId) => {
    if (dragging?.moved || closingId || suppressClickId === appId) return
    openApp(appId)
  }

  return (
    <section className="task-switcher task-switcher-page" aria-label="Task switcher">
      <header className="task-switcher-header">
        <strong>Open apps</strong>
        <span>Swipe a card up to close it</span>
      </header>
      <div className="task-stack">
        {apps.length === 0 ? (
          <div className="empty-task-card"><AppWindow size={42} /><span>No background apps</span></div>
        ) : (
          apps.map((app, index) => (
            <article
              className={`task-card ${app.id === activeApp ? 'active' : ''} ${closingId === app.id ? 'closing' : ''}`}
              key={app.id}
              style={{ '--task-index': index, '--drag-y': `${dragging?.id === app.id ? dragging.offsetY : 0}px` }}
              onClick={() => selectApp(app.id)}
              onPointerDown={(event) => beginDrag(event, app.id)}
              onPointerMove={moveDrag}
              onPointerUp={(event) => finishDrag(event, app.id)}
              onPointerCancel={() => setDragging(null)}
            >
              <header>
                <AppIcon app={app} compact />
                <strong>{app.name}</strong>
                <button type="button" onPointerDown={(event) => event.stopPropagation()} onClick={(event) => {
                  event.stopPropagation()
                  closeWithAnimation(app.id)
                }} aria-label={`Close ${app.name}`}><X size={16} /></button>
              </header>
              <div className="task-preview"><TaskPreview app={app} /></div>
            </article>
          ))
        )}
      </div>
      <div className="task-dock">
        {apps.map((app) => (
          <button key={app.id} onClick={() => openApp(app.id)} className={app.id === activeApp ? 'active' : ''}>
            <AppIcon app={app} />
            <span>{app.short || app.name}</span>
          </button>
        ))}
      </div>
      <button className="task-home" onClick={goHome}>Home</button>
    </section>
  )
}

function TaskPreview({ app }) {
  if (app.id === 'gallery') {
    return <div className="preview-gallery"><div className="calendar-chip">Photos<br /><strong>3</strong></div><img src={assetPath('/references/lockscreen.jpg')} alt="" /><div className="calendar-chip muted">Albums</div></div>
  }
  if (app.id === 'browser') return <div className="preview-browser"><Globe2 size={42} /><span>Web page ready</span></div>
  if (app.id === 'messaging') return <div className="preview-browser"><MessageSquare size={42} /><span>Recent chats</span></div>
  return <div className="preview-generic"><AppIcon app={app} /><span>{app.name}</span></div>
}

function initials(name) {
  return name.split(' ').map((part) => part[0]).join('').slice(0, 2).toUpperCase()
}

function normalizeUrl(value) {
  const trimmed = value.trim()
  if (!trimmed) return 'https://start.duckduckgo.com'
  if (/^https?:\/\//i.test(trimmed)) return trimmed
  if (trimmed.includes('.') && !trimmed.includes(' ')) return `https://${trimmed}`
  return `https://duckduckgo.com/?q=${encodeURIComponent(trimmed)}`
}

function hostLabel(url) {
  try {
    return new URL(url).hostname.replace(/^www\./, '')
  } catch {
    return url
  }
}

function BrowserApp({ data, updateData, notify }) {
  const [address, setAddress] = useState(data.browser.tabs.find((item) => item.id === data.browser.activeTab)?.url || 'https://start.duckduckgo.com')
  const [menuOpen, setMenuOpen] = useState(false)
  const [panel, setPanel] = useState(null)
  const [frameKey, setFrameKey] = useState(0)
  const browser = data.browser
  const tab = browser.tabs.find((item) => item.id === browser.activeTab) || browser.tabs[0]

  const patchBrowser = (patch) => updateData((current) => ({ ...current, browser: { ...current.browser, ...patch } }))
  const setTabs = (tabs, activeTab = browser.activeTab) => patchBrowser({ tabs, activeTab })

  const navigate = (target) => {
    const nextUrl = normalizeUrl(target)
    const tabs = browser.tabs.map((item) => {
      if (item.id !== browser.activeTab) return item
      const history = [...item.history.slice(0, item.index + 1), nextUrl]
      return { ...item, url: nextUrl, title: hostLabel(nextUrl), history, index: history.length - 1 }
    })
    patchBrowser({ tabs, history: [nextUrl, ...browser.history.filter((url) => url !== nextUrl)].slice(0, 30) })
    setAddress(nextUrl)
    setMenuOpen(false)
    setPanel(null)
    setFrameKey((key) => key + 1)
  }

  const moveHistory = (direction) => {
    const tabs = browser.tabs.map((item) => {
      if (item.id !== browser.activeTab) return item
      const index = Math.min(Math.max(item.index + direction, 0), item.history.length - 1)
      setAddress(item.history[index])
      return { ...item, index, url: item.history[index], title: hostLabel(item.history[index]) }
    })
    setTabs(tabs)
    setFrameKey((key) => key + 1)
  }

  const addTab = (privateTab = false) => {
    const id = Date.now()
    const next = { id, url: 'https://start.duckduckgo.com', title: privateTab ? 'Private tab' : 'DuckDuckGo', history: ['https://start.duckduckgo.com'], index: 0, private: privateTab }
    setTabs([...browser.tabs, next], id)
    setMenuOpen(false)
    notify(privateTab ? 'Private tab opened' : 'New tab opened')
  }

  const closeTab = (id) => {
    if (browser.tabs.length === 1) return
    const nextTabs = browser.tabs.filter((item) => item.id !== id)
    if (id === browser.activeTab) setAddress(nextTabs[0].url)
    setTabs(nextTabs, id === browser.activeTab ? nextTabs[0].id : browser.activeTab)
  }

  const toggleBookmark = () => {
    const exists = browser.bookmarks.includes(tab.url)
    patchBrowser({ bookmarks: exists ? browser.bookmarks.filter((url) => url !== tab.url) : [...browser.bookmarks, tab.url] })
    notify(exists ? 'Bookmark removed' : 'Bookmark saved')
  }

  const menuItems = [
    { icon: Plus, label: 'New window', action: () => addTab(false) },
    { icon: Shield, label: 'New private window', action: () => addTab(true) },
    { icon: Upload, label: 'Share', action: () => navigator.clipboard?.writeText(tab.url).then(() => notify('URL copied')) },
    { icon: BookOpen, label: 'Bookmarks', action: () => setPanel(panel === 'bookmarks' ? null : 'bookmarks') },
    { icon: History, label: 'History', action: () => setPanel(panel === 'history' ? null : 'history') },
    { icon: Search, label: 'Find in page', action: () => notify('Find is simulated in this embedded demo') },
    { icon: Download, label: 'Downloads', action: () => setPanel(panel === 'downloads' ? null : 'downloads') },
    { icon: Settings, label: 'Settings', action: () => setPanel(panel === 'settings' ? null : 'settings') },
  ]

  return (
    <div className="browser-app">
      <header className="morph-topbar">
        <button className="morph-back" onClick={() => moveHistory(-1)} disabled={tab.index === 0} title="Back"><ArrowLeft size={18} /></button>
        <form className="morph-address" onSubmit={(event) => {
          event.preventDefault()
          navigate(address)
        }}>
          <span className="duck-badge">D</span>
          <Lock size={17} />
          <input name="address" value={address} onChange={(event) => setAddress(event.target.value)} aria-label="Address" />
          <button type="button" className={browser.bookmarks.includes(tab.url) ? 'bookmarked' : ''} onClick={toggleBookmark} title="Bookmark"><Star size={18} /></button>
        </form>
        <button className="morph-menu-button" onClick={() => setMenuOpen(!menuOpen)} aria-label="Browser menu" title="Menu"><Menu size={24} /></button>
      </header>
      <div className="morph-tabs" aria-label="Browser tabs">
        {browser.tabs.map((item) => (
          <button className={item.id === browser.activeTab ? 'active' : ''} key={item.id} onClick={() => {
            patchBrowser({ activeTab: item.id })
            setAddress(item.url)
            setPanel(null)
          }}>
            {item.private ? <Shield size={14} /> : <Globe2 size={14} />}
            {item.title || hostLabel(item.url)}
            <span onClick={(event) => {
              event.stopPropagation()
              closeTab(item.id)
            }}>x</span>
          </button>
        ))}
        <button onClick={() => addTab(false)} title="New tab"><Plus size={16} /></button>
      </div>
      <section className="morph-viewport">
        {menuOpen && (
          <aside className="morph-menu" aria-label="Browser menu">
            {menuItems.map((item) => {
              const Icon = item.icon
              return <button key={item.label} onClick={item.action}><Icon size={22} />{item.label}</button>
            })}
          </aside>
        )}
        {panel && <BrowserPanel panel={panel} browser={browser} navigate={navigate} closeTab={closeTab} />}
        <iframe key={`${tab.url}-${frameKey}`} title={`Website: ${hostLabel(tab.url)}`} src={tab.url} referrerPolicy="no-referrer" sandbox="allow-forms allow-modals allow-popups allow-popups-to-escape-sandbox allow-same-origin allow-scripts" />
        <div className="iframe-note">
          <strong>{hostLabel(tab.url)}</strong>
          <span>Some sites block embedded browsing. Use the external-open button when a page refuses to load here.</span>
        </div>
      </section>
    </div>
  )
}

function BrowserPanel({ panel, browser, navigate, closeTab }) {
  const list = panel === 'bookmarks' ? browser.bookmarks : panel === 'history' ? browser.history : []
  return (
    <div className="browser-panel">
      <h3>{panel}</h3>
      {panel === 'tabs' && browser.tabs.map((tab) => (
        <button key={tab.id} onClick={() => navigate(tab.url)}><Globe2 size={17} />{tab.title}<X size={15} onClick={(event) => {
          event.stopPropagation()
          closeTab(tab.id)
        }} /></button>
      ))}
      {(panel === 'bookmarks' || panel === 'history') && list.map((url) => (
        <button key={url} onClick={() => navigate(url)}><Globe2 size={17} />{url}</button>
      ))}
      {panel === 'downloads' && browser.downloads.map((file) => (
        <button key={file.id}><Download size={17} />{file.name}<small>{file.size}</small></button>
      ))}
      {panel === 'settings' && <p>DuckDuckGo search, iframe fallback notices, bookmarks, tabs, downloads, and private tabs are enabled for this demo.</p>}
    </div>
  )
}

function GalleryApp({ data, updateData, notify }) {
  const [album, setAlbum] = useState('All')
  const [selectedId, setSelectedId] = useState(data.gallery.find((image) => !image.deleted)?.id)
  const visible = data.gallery.filter((image) => !image.deleted && (album === 'All' || image.album === album))
  const selected = data.gallery.find((image) => image.id === selectedId) || visible[0]
  const albums = ['All', 'Favorites', ...new Set(data.gallery.filter((image) => !image.deleted).map((image) => image.album))]
  const albumImages = album === 'Favorites' ? data.gallery.filter((image) => image.favorite && !image.deleted) : visible

  const updateGallery = (gallery) => updateData((current) => ({ ...current, gallery }))
  const uploadImage = (event) => {
    const file = event.target.files?.[0]
    if (!file) return
    const reader = new FileReader()
    reader.onload = () => {
      const item = { id: `upload-${Date.now()}`, src: reader.result, label: file.name, album: 'Uploads', favorite: false, deleted: false, date: new Date().toISOString().slice(0, 10) }
      updateGallery([item, ...data.gallery])
      setSelectedId(item.id)
      notify('Photo added to Gallery')
    }
    reader.readAsDataURL(file)
  }

  return (
    <div className="gallery-app rich-app">
      <div className="app-tabs">
        {albums.map((item) => <button className={album === item ? 'active' : ''} key={item} onClick={() => setAlbum(item)}>{item}</button>)}
      </div>
      {selected && (
        <figure className="photo-preview">
          <img src={selected.src} alt={selected.label} />
          <figcaption>
            <strong>{selected.label}</strong>
            <span>{selected.album} · {selected.date}</span>
            <div>
              <button onClick={() => updateGallery(data.gallery.map((image) => image.id === selected.id ? { ...image, favorite: !image.favorite } : image))}>{selected.favorite ? 'Unfavorite' : 'Favorite'}</button>
              <button className="danger" onClick={() => updateGallery(data.gallery.map((image) => image.id === selected.id ? { ...image, deleted: true } : image))}>Delete</button>
            </div>
          </figcaption>
        </figure>
      )}
      <label className="upload-button"><Upload size={17} /> Import photo<input type="file" accept="image/*" onChange={uploadImage} /></label>
      <div className="photo-grid">
        {albumImages.map((image) => (
          <button key={image.id} onClick={() => setSelectedId(image.id)} className={selected?.id === image.id ? 'selected' : ''}><img src={image.src} alt={image.label} /></button>
        ))}
      </div>
    </div>
  )
}

function CameraApp({ data, updateData, notify }) {
  const videoRef = useRef(null)
  const canvasRef = useRef(null)
  const [streaming, setStreaming] = useState(false)
  const [cameraError, setCameraError] = useState('')
  const [flash, setFlash] = useState(false)

  useEffect(() => {
    let stream
    navigator.mediaDevices?.getUserMedia?.({ video: true }).then((mediaStream) => {
      stream = mediaStream
      if (videoRef.current) videoRef.current.srcObject = mediaStream
      setStreaming(true)
    }).catch(() => setCameraError('Camera permission unavailable. Mock viewfinder is active.'))
    return () => stream?.getTracks().forEach((track) => track.stop())
  }, [])

  const capture = () => {
    let src = assetPath('/references/lockscreen.jpg')
    if (streaming && videoRef.current && canvasRef.current) {
      const canvas = canvasRef.current
      canvas.width = videoRef.current.videoWidth || 720
      canvas.height = videoRef.current.videoHeight || 960
      canvas.getContext('2d').drawImage(videoRef.current, 0, 0, canvas.width, canvas.height)
      src = canvas.toDataURL('image/png')
    }
    const item = { id: `shot-${Date.now()}`, src, label: `Camera ${new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}`, album: 'Camera', favorite: false, deleted: false, date: new Date().toISOString().slice(0, 10) }
    updateData((current) => ({ ...current, gallery: [item, ...current.gallery] }))
    notify('Photo saved to Gallery')
  }

  return (
    <div className="camera-app">
      <div className={`viewfinder ${flash ? 'flash-on' : ''}`}>
        <video ref={videoRef} autoPlay playsInline muted />
        {!streaming && <div className="mock-lens"><Camera size={64} /><span>{cameraError || 'Starting camera...'}</span></div>}
        <div className="focus-ring" />
      </div>
      <canvas ref={canvasRef} hidden />
      <div className="camera-controls">
        <span>{data.gallery.filter((image) => image.album === 'Camera').length} photos</span>
        <button onClick={() => setFlash(!flash)}>{flash ? 'Flash on' : 'Flash off'}</button>
        <button onClick={capture} className="shutter" aria-label="Take photo" />
        <button>Front</button>
      </div>
    </div>
  )
}

function MessagingApp({ data, updateData, notify }) {
  const [activeId, setActiveId] = useState(data.conversations[0]?.id)
  const [draft, setDraft] = useState('')
  const [showDetail, setShowDetail] = useState(false)
  const active = data.conversations.find((chat) => chat.id === activeId) || data.conversations[0]
  const contact = data.contacts.find((item) => item.id === active?.contactId)

  const send = (event) => {
    event.preventDefault()
    const text = draft.trim()
    if (!text || !active) return
    updateData((current) => ({
      ...current,
      conversations: current.conversations.map((chat) => chat.id === active.id ? {
        ...chat,
        messages: [...chat.messages, { from: 'me', text, time: new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }) }, { from: 'them', text: 'Auto-reply: message received in the local demo.', time: 'now' }],
        unread: 0,
      } : chat),
    }))
    setDraft('')
    notify('Message sent')
  }

  const selectChat = (id) => {
    setActiveId(id)
    setShowDetail(true)
  }

  const goBack = () => setShowDetail(false)

  return (
    <div className="messaging-app split-app">
      <aside className={`side-list ${showDetail ? 'mobile-hidden' : ''}`}>
        <label className="mini-search"><Search size={15} /><input placeholder="Search messages" /></label>
        {data.conversations.map((chat) => {
          const person = data.contacts.find((item) => item.id === chat.contactId)
          return <button className={chat.id === active?.id ? 'active' : ''} key={chat.id} onClick={() => selectChat(chat.id)}><span className="avatar">{initials(person?.name || '?')}</span><strong>{person?.name}</strong>{chat.unread > 0 && <em>{chat.unread}</em>}</button>
        })}
      </aside>
      <section className={`chat-pane ${!showDetail ? 'mobile-hidden' : ''}`}>
        <button className="mobile-back" onClick={goBack}><ArrowLeft size={20} />Back</button>
        <header><strong>{contact?.name}</strong><small>{contact?.phone}</small></header>
        <div className="chat-list">
          {active?.messages.map((message, index) => <p className={message.from} key={`${message.text}-${index}`}><span>{message.text}</span><small>{message.time}</small></p>)}
        </div>
        <form className="compose" onSubmit={send}>
          <input value={draft} onChange={(event) => setDraft(event.target.value)} placeholder="Message" />
          <button>Send</button>
        </form>
      </section>
    </div>
  )
}

function PhoneApp({ data, updateData, notify }) {
  const [number, setNumber] = useState('')
  const [tab, setTab] = useState('Dialer')
  const [inCall, setInCall] = useState(null)
  const keys = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '*', '0', '#']
  const call = (targetNumber = number, name = targetNumber) => {
    if (!targetNumber) return
    setInCall({ name, number: targetNumber, seconds: 0 })
    updateData((current) => ({ ...current, callLog: [{ id: Date.now(), name, number: targetNumber, type: 'outgoing', time: 'Just now' }, ...current.callLog] }))
    notify(`Calling ${name}`)
  }

  useEffect(() => {
    if (!inCall) return undefined
    const timer = window.setInterval(() => setInCall((current) => current ? { ...current, seconds: current.seconds + 1 } : current), 1000)
    return () => window.clearInterval(timer)
  }, [inCall])

  if (inCall) {
    return (
      <div className="active-call">
        <span className="avatar large">{initials(inCall.name)}</span>
        <h2>{inCall.name}</h2>
        <p>{Math.floor(inCall.seconds / 60)}:{String(inCall.seconds % 60).padStart(2, '0')}</p>
        <div className="call-tools"><button>Mute</button><button>Speaker</button><button>Keypad</button></div>
        <button className="hangup" onClick={() => setInCall(null)}>End call</button>
      </div>
    )
  }

  return (
    <div className="phone-app rich-app">
      <div className="app-tabs">{['Dialer', 'Recents', 'Contacts'].map((item) => <button className={tab === item ? 'active' : ''} key={item} onClick={() => setTab(item)}>{item}</button>)}</div>
      {tab === 'Dialer' && <>
        <div className="dial-number">{number || 'Tap a number'}</div>
        <div className="dialpad">{keys.map((key) => <button key={key} onClick={() => setNumber(`${number}${key}`)}>{key}</button>)}</div>
        <button className="call-button" onClick={() => call()}><Phone size={20} />Call</button>
      </>}
      {tab === 'Recents' && <div className="list-stack">{data.callLog.map((callItem) => <button key={callItem.id} onClick={() => call(callItem.number, callItem.name)}><Phone size={18} /><strong>{callItem.name}</strong><small>{callItem.type} · {callItem.time}</small></button>)}</div>}
      {tab === 'Contacts' && <div className="list-stack">{data.contacts.map((person) => <button key={person.id} onClick={() => call(person.phone, person.name)}><span className="avatar">{initials(person.name)}</span><strong>{person.name}</strong><small>{person.phone}</small></button>)}</div>}
    </div>
  )
}

function SettingsApp({ data, updateData, resetDemo, darkMode }) {
  const updateStatus = (patch) => updateData((current) => ({ ...current, status: { ...current.status, ...patch } }))
  const toggleDarkMode = () => updateData((current) => ({ ...current, darkMode: !current.darkMode }))
  const rows = [
    ['wifi', 'Wi-Fi', 'Connected to DemoNet'],
    ['bluetooth', 'Bluetooth', 'PinePhone accessories'],
    ['location', 'Location', data.status.location ? 'Available to Weather and uNav' : 'Off'],
    ['sound', 'Sound', data.status.sound ? 'Ringer and media enabled' : 'Muted'],
    ['batterySaver', 'Battery saver', 'Dims background activity'],
    ['notifications', 'Notifications', 'Banners and badges'],
  ]

  return (
    <div className={`settings-app rich-app ${darkMode ? 'dark-mode' : 'light-mode'}`}>
      <section className="settings-section"><h3>Quick settings</h3>{rows.map(([key, label, meta]) => (
        <label className="settings-row" key={key}><span><strong>{label}</strong><small>{meta}</small></span><input type="checkbox" checked={Boolean(data.status[key])} onChange={() => updateStatus({ [key]: !data.status[key] })} /></label>
      ))}</section>
      <section className="settings-section">
        <h3>Display</h3>
        <label className="settings-row" onClick={toggleDarkMode}>
          <span><strong>Dark mode</strong><small>{data.darkMode ? 'On' : 'Off'}</small></span>
          <button className={`toggle-switch ${data.darkMode ? 'on' : ''}`}>
            <span className="toggle-knob" />
          </button>
        </label>
        <div className="settings-row"><span><strong>Demo size</strong><small>{data.demoWidth}x{data.demoHeight}</small></span><Moon size={22} /></div>
      </section>
      <section className="settings-section"><h3>Storage</h3><div className="storage-bar"><span style={{ width: `${Math.min(88, 24 + data.gallery.length * 4 + data.files.length * 2)}%` }} /></div><small>{data.gallery.length} photos · {data.files.length} files · {data.installedApps.length} installed apps</small></section>
      <section className="settings-section"><h3>Apps</h3>{[...appCatalog, ...storeApps.filter((app) => data.installedApps.includes(app.id))].map((app) => <div className="settings-row" key={app.id}><span><strong>{app.name}</strong><small>Local demo permissions</small></span><AppIcon app={app} compact /></div>)}</section>
      <section className="settings-section"><h3>About</h3><p>Ubuntu Experience · client-side daily-driver simulation · Ubuntu font by Dalton Maag.</p><p><a href="https://kenneth-cho-infosec.github.io/UbuntuExperience/" target="_blank" rel="noopener">Live Demo</a> · <a href="https://github.com/Kenneth-Cho-InfoSec/UbuntuExperience" target="_blank" rel="noopener">GitHub</a></p><button className="danger full-button" onClick={resetDemo}>Reset demo data</button></section>
    </div>
  )
}

function TerminalApp({ data, openApp, installApp }) {
  const [lines, setLines] = useState(['phablet@ubuntu:~$ welcome', 'Try: help, apps, open browser, install dekko, notes, contacts, weather, ls, date, clear, whoami, uname, uptime, df, free, hostname, pwd, echo'])
  const [command, setCommand] = useState('')
  const runCommand = (event) => {
    event.preventDefault()
    const next = command.trim()
    if (next === 'clear') {
      setLines([])
      setCommand('')
      return
    }
    let output = ''
    const [cmd, ...args] = next.split(' ')
    const arg = args.join(' ')
    if (cmd === 'help') output = 'Commands: help, apps, open <app>, install <app>, notes, contacts, weather, status, ls, cat <file>, date, clear, whoami, uname, uptime, df, free, hostname, pwd, echo <text>'
    else if (cmd === 'apps') output = `Installed: ${[...appCatalog.map((app) => app.id), ...data.installedApps].join(', ')}`
    else if (cmd === 'open' && arg) {
      openApp(arg)
      output = `Opening ${arg}`
    } else if (cmd === 'install' && arg) {
      installApp(arg)
      output = `Installing ${arg} from OpenStore`
    } else if (cmd === 'notes') output = data.notes.map((note) => note.title).join('\n')
    else if (cmd === 'contacts') output = data.contacts.map((person) => `${person.name} ${person.phone}`).join('\n')
    else if (cmd === 'weather') output = `${data.weather.location}: ${data.weather.temp}°${data.weather.unit}, ${data.weather.condition}`
    else if (cmd === 'status') output = `Wi-Fi ${data.status.wifi ? 'on' : 'off'}, ${data.status.network}, battery ${data.status.battery}%`
    else if (cmd === 'ls') output = data.files.map((file) => file.name).join('\n')
    else if (cmd === 'cat' && arg) {
      const file = data.files.find((f) => f.name.toLowerCase() === arg.toLowerCase() || f.id === arg)
      output = file?.content || `${arg}: No such file`
    } else if (cmd === 'date') output = new Date().toLocaleString()
    else if (cmd === 'whoami') output = 'phablet'
    else if (cmd === 'uname' && arg === '-a') output = 'Ubuntu 24.04 LTS aarch64 GNU/Linux'
    else if (cmd === 'uname') output = 'Linux'
    else if (cmd === 'uptime') {
      const hours = Math.floor(Math.random() * 24)
      const mins = Math.floor(Math.random() * 60)
      output = ` ${hours}:${mins.toString().padStart(2, '0')} up 1 day, ${Math.floor(Math.random() * 3) + 1}:${Math.floor(Math.random() * 60)}, 2 users, load average: 0.1, 0.2, 0.3`
    } else if (cmd === 'df') output = 'Filesystem     1K-blocks    Used Available Use% Mounted on\n/dev/mmcblk0p2   31457280 8388608  20971520  29% /\ntmpfs             1024000       0   1024000   0% /dev/shm'
    else if (cmd === 'free') output = '              total        used        free      shared  buff/cache   available\nMem:        2048000      512000     1024000       25600      512000      1400000\nSwap:        512000           0      512000'
    else if (cmd === 'hostname') output = 'ubuntu-phablet'
    else if (cmd === 'pwd') output = '/home/phablet'
    else if (cmd === 'echo') output = arg
    else if (next) output = `${next}: command not found`
    setLines([...lines, `phablet@ubuntu:~$ ${next}`, output])
    setCommand('')
  }

  return (
    <div className="terminal-app">
      <div className="terminal-lines">{lines.map((line, index) => <pre key={`${line}-${index}`}>{line}</pre>)}</div>
      <form onSubmit={runCommand}><span>$</span><input value={command} onChange={(event) => setCommand(event.target.value)} aria-label="Terminal command" /></form>
    </div>
  )
}

function NotesApp({ data, updateData, notify }) {
  const [activeId, setActiveId] = useState(data.notes.find((note) => !note.deleted)?.id)
  const [search, setSearch] = useState('')
  const [showDetail, setShowDetail] = useState(false)
  const notes = data.notes.filter((note) => !note.deleted && note.title.toLowerCase().includes(search.toLowerCase()))
  const active = data.notes.find((note) => note.id === activeId) || notes[0]
  const save = (patch) => updateData((current) => ({ ...current, notes: current.notes.map((note) => note.id === active.id ? { ...note, ...patch, updated: 'Just now' } : note) }))
  const addNote = () => {
    const note = { id: `note-${Date.now()}`, title: 'New note', body: '', tags: [], pinned: false, deleted: false, updated: 'Just now' }
    updateData((current) => ({ ...current, notes: [note, ...current.notes] }))
    setActiveId(note.id)
  }

  const selectNote = (id) => {
    setActiveId(id)
    setShowDetail(true)
  }

  const goBack = () => setShowDetail(false)

  return (
    <div className="notes-split split-app">
      <aside className={`side-list ${showDetail ? 'mobile-hidden' : ''}`}>
        <label className="mini-search"><Search size={15} /><input value={search} onChange={(event) => setSearch(event.target.value)} placeholder="Search notes" /></label>
        <button onClick={addNote}><Plus size={17} />New note</button>
        {notes.map((note) => <button className={note.id === active?.id ? 'active' : ''} key={note.id} onClick={() => selectNote(note.id)}><strong>{note.pinned ? '★ ' : ''}{note.title}</strong><small>{note.updated}</small></button>)}
      </aside>
      {active && (
        <section className={`note-editor ${!showDetail ? 'mobile-hidden' : ''}`}>
          <button className="mobile-back" onClick={goBack}><ArrowLeft size={20} />Back</button>
          <input value={active.title} onChange={(event) => save({ title: event.target.value })} />
          <textarea value={active.body} onChange={(event) => save({ body: event.target.value })} />
          <div className="note-actions">
            <button onClick={() => save({ pinned: !active.pinned })}>{active.pinned ? 'Unpin' : 'Pin'}</button>
            <button onClick={() => {
              updateData((current) => ({ ...current, files: [{ id: `note-file-${Date.now()}`, name: `${active.title}.txt`, type: 'text', path: '/Documents', parent: 'documents', app: 'notes', content: active.body }, ...current.files] }))
              notify('Note exported to Files')
            }}>Export</button>
            <button className="danger" onClick={() => save({ deleted: true })}>Delete</button>
          </div>
        </section>
      )}
    </div>
  )
}

function WeatherApp({ data, updateData }) {
  const weather = data.weather
  const toggleUnit = () => updateData((current) => {
    const toF = current.weather.unit === 'C'
    const temp = toF ? Math.round((current.weather.temp * 9) / 5 + 32) : Math.round(((current.weather.temp - 32) * 5) / 9)
    return { ...current, weather: { ...current.weather, unit: toF ? 'F' : 'C', temp } }
  })
  return (
    <div className="weather-app">
      <CloudSun size={78} />
      <strong>{weather.temp}°{weather.unit}</strong>
      <span>{weather.location} · {weather.condition}</span>
      <button onClick={toggleUnit}>Use °{weather.unit === 'C' ? 'F' : 'C'}</button>
      <div className="forecast">{weather.hourly.map((temp, index) => <p key={index}>{index === 0 ? 'Now' : `${index + 14}:00`}<br />{temp}°</p>)}</div>
      <div className="daily-forecast">{weather.daily.map(([day, temp, condition]) => <p key={day}><strong>{day}</strong><b>{temp}° <small>{condition}</small></b></p>)}</div>
    </div>
  )
}

function StoreApp({ openApp, installedApps, installApp, removeApp }) {
  const [filter, setFilter] = useState('All')
  const [query, setQuery] = useState('')
  const [selectedId, setSelectedId] = useState(storeApps[0].id)
  const [installing, setInstalling] = useState(null)
  const categories = ['All', ...new Set(storeApps.map((app) => app.category))]
  const visibleApps = storeApps.filter((app) => (filter === 'All' || app.category === filter) && app.name.toLowerCase().includes(query.toLowerCase()))
  const selected = storeApps.find((app) => app.id === selectedId) || storeApps[0]
  const startInstall = (id) => {
    setInstalling(id)
    window.setTimeout(() => {
      installApp(id)
      setInstalling(null)
    }, 650)
  }

  return (
    <div className="store-app">
      <header className="store-hero"><span className="store-bag"><ShoppingBag size={30} /></span><div><h2>OpenStore</h2><p>Install demo apps into the Ubuntu Experience drawer.</p></div></header>
      <label className="mini-search"><Search size={16} /><input value={query} onChange={(event) => setQuery(event.target.value)} placeholder="Search OpenStore" /></label>
      <div className="store-filters" aria-label="OpenStore categories">{categories.map((category) => <button className={filter === category ? 'active' : ''} key={category} onClick={() => setFilter(category)}>{category}</button>)}</div>
      <section className="store-detail"><AppIcon app={selected} /><div><span>Featured app</span><strong>{selected.name}</strong><p>{selected.summary}</p><small>{selected.category} · {selected.rating} stars · {selected.size}</small><p>Permissions: {selected.permissions.join(', ')}</p></div></section>
      <div className="store-list">{visibleApps.map((storeApp) => {
        const installed = installedApps.includes(storeApp.id)
        const busy = installing === storeApp.id
        return (
          <article key={storeApp.id} onClick={() => setSelectedId(storeApp.id)}>
            <AppIcon app={storeApp} compact />
            <div className="store-copy"><strong>{storeApp.name}</strong><small>{storeApp.category} · {storeApp.rating} stars · {storeApp.size}</small><p>{storeApp.summary}</p>{busy && <span className="install-progress"><span /></span>}</div>
            <div className="store-actions">{installed ? <><button onClick={(event) => {
              event.stopPropagation()
              openApp(storeApp.id)
            }}>Open</button><button className="ghost danger" onClick={(event) => {
              event.stopPropagation()
              removeApp(storeApp.id)
            }}><Trash2 size={16} /></button></> : <button onClick={(event) => {
              event.stopPropagation()
              startInstall(storeApp.id)
            }} disabled={busy}>{busy ? 'Installing' : 'Install'}</button>}</div>
          </article>
        )
      })}</div>
    </div>
  )
}

function ContactsApp({ data, updateData, openApp, notify }) {
  const [selectedId, setSelectedId] = useState(data.contacts[0]?.id)
  const [query, setQuery] = useState('')
  const [showDetail, setShowDetail] = useState(false)
  const selected = data.contacts.find((person) => person.id === selectedId) || data.contacts[0]
  const contacts = data.contacts.filter((person) => person.name.toLowerCase().includes(query.toLowerCase()))
  const save = (patch) => updateData((current) => ({ ...current, contacts: current.contacts.map((person) => person.id === selected.id ? { ...person, ...patch } : person) }))
  const addContact = () => {
    const contact = { id: `contact-${Date.now()}`, name: 'New Contact', phone: '+44 7700 900000', email: 'new@example.test', favorite: false, group: 'Personal' }
    updateData((current) => ({ ...current, contacts: [contact, ...current.contacts] }))
    setSelectedId(contact.id)
  }

  const selectContact = (id) => {
    setSelectedId(id)
    setShowDetail(true)
  }

  const goBack = () => setShowDetail(false)

  return (
    <div className="contacts-app split-app">
      <aside className={`side-list ${showDetail ? 'mobile-hidden' : ''}`}>
        <label className="mini-search"><Search size={15} /><input value={query} onChange={(event) => setQuery(event.target.value)} placeholder="Search contacts" /></label>
        <button onClick={addContact}><Plus size={17} />New contact</button>
        {contacts.map((person) => <button className={person.id === selected?.id ? 'active' : ''} key={person.id} onClick={() => selectContact(person.id)}><span className="avatar">{initials(person.name)}</span><strong>{person.name}</strong><small>{person.group}</small></button>)}
      </aside>
      {selected && (
        <section className={`contact-detail ${!showDetail ? 'mobile-hidden' : ''}`}>
          <button className="mobile-back" onClick={goBack}><ArrowLeft size={20} />Back</button>
          <span className="avatar large">{initials(selected.name)}</span>
          <input value={selected.name} onChange={(event) => save({ name: event.target.value })} />
          <input value={selected.phone} onChange={(event) => save({ phone: event.target.value })} />
          <input value={selected.email} onChange={(event) => save({ email: event.target.value })} />
          <div className="contact-actions">
            <button onClick={() => openApp('phone')}><Phone size={17} />Call</button>
            <button onClick={() => openApp('messaging')}><MessageSquare size={17} />Message</button>
            <button onClick={() => {
              save({ favorite: !selected.favorite })
              notify(selected.favorite ? 'Favorite removed' : 'Favorite contact saved')
            }}><Star size={17} />{selected.favorite ? 'Unfavorite' : 'Favorite'}</button>
          </div>
        </section>
      )}
    </div>
  )
}

function MusicApp() {
  const [activeTab, setActiveTab] = useState('library')
  const [currentTrack, setCurrentTrack] = useState(0)
  const [isPlaying, setIsPlaying] = useState(false)
  const [progress, setProgress] = useState(42)
  const [shuffle, setShuffle] = useState(false)
  const [repeat, setRepeat] = useState('off')
  const [searchQuery, setSearchQuery] = useState('')

  const sampleTracks = [
    { id: 1, title: 'Midnight Dreams', artist: 'Luna Eclipse', album: 'Starlight', duration: '3:42', cover: null },
    { id: 2, title: 'Electric Pulse', artist: 'Neon Wave', album: 'Synth City', duration: '4:15', cover: null },
    { id: 3, title: 'Ocean Breeze', artist: 'Coastal', album: 'Summer', duration: '3:28', cover: null },
    { id: 4, title: 'Urban Jungle', artist: 'City Lights', album: 'Night Life', duration: '3:55', cover: null },
    { id: 5, title: 'Starlight Serenade', artist: 'Luna Eclipse', album: 'Starlight', duration: '4:02', cover: null },
    { id: 6, title: 'Digital Love', artist: 'Neon Wave', album: 'Synth City', duration: '3:38', cover: null },
  ]

  const albums = [
    { id: 1, name: 'Starlight', artist: 'Luna Eclipse', cover: null },
    { id: 2, name: 'Synth City', artist: 'Neon Wave', cover: null },
    { id: 3, name: 'Summer', artist: 'Coastal', cover: null },
    { id: 4, name: 'Night Life', artist: 'City Lights', cover: null },
  ]

  const artists = ['Luna Eclipse', 'Neon Wave', 'Coastal', 'City Lights']

  const filteredTracks = searchQuery
    ? sampleTracks.filter(t => t.title.toLowerCase().includes(searchQuery.toLowerCase()) || t.artist.toLowerCase().includes(searchQuery.toLowerCase()))
    : sampleTracks

  const formatTime = (seconds) => {
    const mins = Math.floor(seconds / 60)
    const secs = Math.floor(seconds % 60)
    return `${mins}:${secs.toString().padStart(2, '0')}`
  }

  const playTrack = (index) => {
    setCurrentTrack(index)
    setIsPlaying(true)
  }

  const togglePlay = () => setIsPlaying(!isPlaying)

  const nextTrack = () => {
    setCurrentTrack((currentTrack + 1) % sampleTracks.length)
  }

  const prevTrack = () => {
    setCurrentTrack((currentTrack - 1 + sampleTracks.length) % sampleTracks.length)
  }

  const cycleRepeat = () => {
    const modes = ['off', 'all', 'one']
    const currentIndex = modes.indexOf(repeat)
    setRepeat(modes[(currentIndex + 1) % modes.length])
  }

  const track = sampleTracks[currentTrack]

  return (
    <div className="music-app">
      <div className="music-tabs">
        <button className={activeTab === 'library' ? 'active' : ''} onClick={() => setActiveTab('library')}>Library</button>
        <button className={activeTab === 'nowplaying' ? 'active' : ''} onClick={() => setActiveTab('nowplaying')}>Now Playing</button>
        <button className={activeTab === 'search' ? 'active' : ''} onClick={() => setActiveTab('search')}>Search</button>
      </div>

      {activeTab === 'library' && (
        <div className="music-library">
          <div className="music-section">
            <h3>Recently Played</h3>
            <div className="music-horiz-scroll">
              {albums.slice(0, 3).map(album => (
                <div key={album.id} className="music-album-card" onClick={() => playTrack(0)}>
                  <div className="music-album-cover"><Music size={28} /></div>
                  <span>{album.name}</span>
                  <small>{album.artist}</small>
                </div>
              ))}
            </div>
          </div>

          <div className="music-section">
            <h3>Songs</h3>
            <div className="music-list">
              {sampleTracks.map((t, index) => (
                <button key={t.id} className={index === currentTrack ? 'active' : ''} onClick={() => playTrack(index)}>
                  <div className="music-track-num">{index + 1}</div>
                  <div className="music-track-info">
                    <strong>{t.title}</strong>
                    <small>{t.artist}</small>
                  </div>
                  <span className="music-track-duration">{t.duration}</span>
                </button>
              ))}
            </div>
          </div>

          <div className="music-section">
            <h3>Albums</h3>
            <div className="music-horiz-scroll">
              {albums.map(album => (
                <div key={album.id} className="music-album-card">
                  <div className="music-album-cover"><Music size={28} /></div>
                  <span>{album.name}</span>
                  <small>{album.artist}</small>
                </div>
              ))}
            </div>
          </div>

          <div className="music-section">
            <h3>Artists</h3>
            <div className="music-horiz-scroll">
              {artists.map(artist => (
                <div key={artist} className="music-artist-card">
                  <div className="music-artist-avatar"><Music size={24} /></div>
                  <span>{artist}</span>
                </div>
              ))}
            </div>
          </div>
        </div>
      )}

      {activeTab === 'nowplaying' && (
        <div className="music-player">
          <div className="now-playing-art">
            <div className="album-art-large"><Music size={72} /></div>
          </div>
          <div className="now-playing-info">
            <h2>{track.title}</h2>
            <p>{track.artist}</p>
            <small>{track.album}</small>
          </div>
          <div className="now-playing-progress">
            <input type="range" min="0" max="100" value={progress} onChange={(e) => setProgress(e.target.value)} className="music-progress" />
            <div className="music-time-display">
              <span>{formatTime((progress / 100) * 200)}</span>
              <span>{formatTime(200)}</span>
            </div>
          </div>
          <div className="now-playing-controls">
            <button className={`music-control-btn ${shuffle ? 'active' : ''}`} onClick={() => setShuffle(!shuffle)}><Shuffle size={18} /></button>
            <button className="music-control-btn" onClick={prevTrack}><SkipBack size={24} /></button>
            <button className="music-control-btn play-btn" onClick={togglePlay}>{isPlaying ? <Pause size={28} /> : <Play size={28} />}</button>
            <button className="music-control-btn" onClick={nextTrack}><SkipForward size={24} /></button>
            <button className={`music-control-btn ${repeat !== 'off' ? 'active' : ''}`} onClick={cycleRepeat}><Repeat size={18} /></button>
          </div>
          <div className="now-playing-extras">
            <button><ListMusic size={20} /></button>
            <button><Volume2 size={20} /></button>
          </div>
        </div>
      )}

      {activeTab === 'search' && (
        <div className="music-search">
          <div className="music-search-bar">
            <Search size={18} />
            <input type="text" placeholder="Search songs, artists, albums..." value={searchQuery} onChange={(e) => setSearchQuery(e.target.value)} />
          </div>
          {searchQuery ? (
            <div className="music-list">
              {filteredTracks.length > 0 ? (
                filteredTracks.map((t) => (
                  <button key={t.id} onClick={() => { playTrack(sampleTracks.indexOf(t)); setActiveTab('nowplaying') }}>
                    <Music size={18} />
                    <div className="music-track-info">
                      <strong>{t.title}</strong>
                      <small>{t.artist}</small>
                    </div>
                  </button>
                ))
              ) : (
                <p className="music-no-results">No results found</p>
              )}
            </div>
          ) : (
            <div className="music-search-suggestions">
              <h3>Recent Searches</h3>
              <p>No recent searches</p>
              <h3>Suggested</h3>
              <div className="music-horiz-scroll">
                {sampleTracks.slice(0, 4).map(t => (
                  <button key={t.id} onClick={() => setSearchQuery(t.title)} className="music-suggestion-chip">{t.title}</button>
                ))}
              </div>
            </div>
          )}
        </div>
      )}
    </div>
  )
}

function FilesApp({ data, updateData, openApp, notify }) {
  const [folder, setFolder] = useState('/')
  const items = data.files.filter((file) => folder === '/' ? file.parent === '/' : file.parent === folder)
  const createFolder = () => {
    const name = `Folder ${data.files.length + 1}`
    updateData((current) => ({ ...current, files: [...current.files, { id: `folder-${Date.now()}`, name, type: 'folder', path: folder, parent: folder, app: 'files' }] }))
    notify('Folder created')
  }
  return (
    <div className="files-app rich-app">
      <div className="file-toolbar"><button onClick={() => setFolder('/')}><Folder size={17} />Root</button><button onClick={createFolder}><Plus size={17} />New folder</button><span>{folder}</span></div>
      {folder !== '/' && <button onClick={() => setFolder('/')}>Back to root</button>}
      {items.map((file) => <button key={file.id} onClick={() => file.type === 'folder' ? setFolder(file.id) : openApp(file.app)}>{file.type === 'folder' ? <Folder size={28} /> : <FileText size={28} />}<span><strong>{file.name}</strong><small>{file.type} · opens with {file.app}</small></span></button>)}
    </div>
  )
}

function DekkoApp({ data, updateData, removeApp, app }) {
  const [selectedId, setSelectedId] = useState(data.mail[0]?.id)
  const selected = data.mail.find((mail) => mail.id === selectedId) || data.mail[0]
  return (
    <div className="mail-app split-app"><aside className="side-list"><button><Plus size={17} />Compose</button>{data.mail.map((mail) => <button className={mail.id === selected?.id ? 'active' : ''} key={mail.id} onClick={() => setSelectedId(mail.id)}><Inbox size={17} /><strong>{mail.subject}</strong><small>{mail.from}</small></button>)}</aside>{selected && <section className="reader-pane"><h2>{selected.subject}</h2><small>{selected.from} · {selected.folder}</small><p>{selected.body}</p><button onClick={() => updateData((current) => ({ ...current, mail: current.mail.map((mail) => mail.id === selected.id ? { ...mail, starred: !mail.starred } : mail) }))}><Star size={17} />{selected.starred ? 'Unstar' : 'Star'}</button><button className="danger" onClick={() => removeApp(app.id)}>Uninstall Dekko</button></section>}</div>
  )
}

function PodbirdApp({ data, removeApp, app }) {
  return <div className="media-app"><div className="album-art"><Mic size={76} /></div><h2>Podbird</h2><p>Podcast queue and offline episodes</p><div className="list-stack">{data.podcasts.map((podcast) => <button key={podcast.id}><Mic size={18} /><strong>{podcast.show}</strong><small>{podcast.episode} · {podcast.progress}% played</small></button>)}</div><button className="danger" onClick={() => removeApp(app.id)}>Uninstall</button></div>
}

function TeleportsApp({ data, openApp, removeApp, app }) {
  return <div className="messaging-app split-app"><aside className="side-list">{data.contacts.slice(0, 3).map((person) => <button key={person.id}><span className="avatar">{initials(person.name)}</span><strong>{person.name}</strong><small>Online</small></button>)}</aside><section className="chat-pane"><header><strong>Teleports</strong><small>Encrypted local demo chat</small></header><div className="chat-list"><p className="them">Welcome to Teleports.</p><p className="me">This feels like a real installed app.</p></div><button onClick={() => openApp('messaging')}>Open system Messaging</button><button className="danger" onClick={() => removeApp(app.id)}>Uninstall</button></section></div>
}

function UNavApp({ data, removeApp, app }) {
  return <div className="unav-app"><div className="map-mock"><Navigation size={64} /><span>Route preview</span></div><div className="list-stack">{data.places.map((place) => <button key={place.id}><MapPin size={18} /><strong>{place.name}</strong><small>{place.detail}</small></button>)}</div><button className="call-button">Start navigation</button><button className="danger" onClick={() => removeApp(app.id)}>Uninstall</button></div>
}

function GenericApp({ app, removeApp }) {
  return (
    <div className="generic-app">
      <AppIcon app={app} />
      <h2>{app.name}</h2>
      <p>{app.summary || 'This app is available in demo mode.'}</p>
      {storeApps.some((storeApp) => storeApp.id === app.id) && <button className="danger" onClick={() => removeApp(app.id)}><Trash2 size={16} />Remove</button>}
    </div>
  )
}

export default App
