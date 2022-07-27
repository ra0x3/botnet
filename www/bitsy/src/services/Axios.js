import axios from 'axios';
import config from '../config';

export const api = axios.create({
  baseURL: `${config.protocol}://${config.bitsy_api_host}:${config.bitsy_api_port}`,
});
