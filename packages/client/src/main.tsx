import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App.tsx'
import './globals.css'
import { HashRouter } from 'react-router-dom'
import { VoteManagementProvider } from '@/context/voteManagement/index.ts'
import { NotificationAlertProvider } from './context/NotificationAlert/NotificationAlert.context.tsx'

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.Fragment>
    <HashRouter>
      <NotificationAlertProvider>
        <VoteManagementProvider>
          <App />
        </VoteManagementProvider>
      </NotificationAlertProvider>
    </HashRouter>
  </React.Fragment>,
)
