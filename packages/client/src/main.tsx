import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App.tsx'
import './globals.css'
import { HashRouter } from 'react-router-dom'
import { VoteManagementProvider } from '@/context/voteManagement/index.ts'
import { NotificationAlertProvider } from './context/NotificationAlert/NotificationAlert.context.tsx'

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <HashRouter>
      <VoteManagementProvider>
        <NotificationAlertProvider>
          <App />
        </NotificationAlertProvider>
      </VoteManagementProvider>
    </HashRouter>
  </React.StrictMode>,
)
