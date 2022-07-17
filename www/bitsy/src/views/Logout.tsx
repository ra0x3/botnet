import React from 'react';
import {routeTo} from '../utils';
export default class Logout extends React.Component {
  componentDidMount() {
    localStorage.removeItem('x-bitsy');
    return routeTo('/');
  }

  render() {
    return null;
  }
}
