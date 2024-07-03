import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App.tsx'
import './globals.css'
import { HashRouter } from 'react-router-dom'
import { VoteManagementProvider } from '@/context/voteManagement/index.ts'
import { NotificationAlertProvider } from './context/NotificationAlert/NotificationAlert.context.tsx'
import { DynamicContextProvider } from '@dynamic-labs/sdk-react-core'

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.Fragment>
    <HashRouter>
      <DynamicContextProvider
        settings={{
          environmentId: import.meta.env.VITE_DYNAMIC_ENV_ID,
          debugError: true,
        }}
      >
        <NotificationAlertProvider>
          <VoteManagementProvider>
            <App />
          </VoteManagementProvider>
        </NotificationAlertProvider>
      </DynamicContextProvider>
    </HashRouter>
  </React.Fragment>,
)
