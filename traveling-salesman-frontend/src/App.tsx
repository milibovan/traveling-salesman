// import { useState } from 'react'
// import reactLogo from './assets/react.svg'
// import viteLogo from '/vite.svg'
import './App.css'
import MapComponent from './components/MapComponent'

function App() {
  // const [count, setCount] = useState(0)

  return (
      <div style={{ padding: '20px', textAlign: 'center' }}>
      <h1>{"TSP Genetic Algorithm Frontend"}</h1>
      <p>{"Select cities and find the optimal tour."}</p>
      
      {/* Render the map component */}
      <MapComponent />
      
    </div>
  )
}

export default App
