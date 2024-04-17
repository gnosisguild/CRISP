import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App.tsx'
import './globals.css'
import { BrowserRouter } from 'react-router-dom'
import { VoteManagementProvider } from '@/context/voteManagement/index.ts'

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <BrowserRouter>
      <VoteManagementProvider>
        <App />
      </VoteManagementProvider>
    </BrowserRouter>
  </React.StrictMode>,
)
