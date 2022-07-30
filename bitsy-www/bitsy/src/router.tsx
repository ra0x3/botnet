import React from 'react';
import {BrowserRouter as Router, Routes, Route} from 'react-router-dom';
import DashboardView from './views/Dashboard';
import LandingView from './views/Landing';
import AccessTokensView from './views/AccessTokens';
import WebhooksView from './views/Webhooks';
import SettingsView from './views/Settings';
import LogoutView from './views/Logout';

class BitsyRouter extends React.Component {
  render() {
    return (
      <Router>
        <Routes>
          <Route path="/dashboard" element={<DashboardView history={{}} match={{}} />} />
          <Route path="/access-tokens" element={<AccessTokensView history={{}} match={{}} />} />
          <Route path="/webhooks" element={<WebhooksView history={{}} match={{}} />} />
          <Route path="/settings" element={<SettingsView history={{}} match={{}} />} />
          <Route path="/logout" element={<LogoutView />} />
          <Route path="/" element={<LandingView history={{}} match={{}} />} />
          <Route path="*" element={<LandingView history={{}} match={{}} />} />
        </Routes>
      </Router>
    );
  }
}

export default BitsyRouter;
