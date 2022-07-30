export default {
  protocol: process.env.REACT_APP_PROTOCOL ?? 'http',
  bitsy_api_host: process.env.REACT_APP_BITSY_API_HOST ?? '0.0.0.0',
  bitsy_api_port: process.env.REACT_APP_BITSY_API_PORT ?? '8000',
};
