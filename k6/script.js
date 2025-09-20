import http from 'k6/http';
import { check } from 'k6';

export const options = {
  stages: [
    { duration: '30s', target: 100 },
    { duration: '1m', target: 200 },
    { duration: '30s', target: 0 },
  ],
  thresholds: {
    http_req_duration: ['p(95)<500'],
  },
};

const BASE = __ENV.BASE_URL || 'http://localhost:8080';

export default function () {
  const payload = JSON.stringify({
    username: `user${__VU}@load.test`,
    password: 'SuperSecret123',
  });
  const res = http.post(`${BASE}/register`, payload, {
    headers: { 'Content-Type': 'application/json' },
  });
  check(res, { 'registered': (r) => r.status === 201 });
}
