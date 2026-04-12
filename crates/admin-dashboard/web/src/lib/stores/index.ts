import { writable, type Readable, type Writable } from 'svelte/store';

export type SystemStats = {
  memory: number;
  virtualMemory: number;
  cpuUsage: number;
  startTime: number;
};

export type TraceLevel = 'TRACE' | 'DEBUG' | 'INFO' | 'WARN' | 'ERROR';

export type Trace = {
  level: TraceLevel;
  message: string | undefined;
  fields: {
    [name: string]: string;
  };
  timestamp: string;
};

export type Plugin = {
  id: string;
  name: string;
  version: string | null;
  description: string | null;
  endpoint: string;
};

const makeWritables = (
  url: URL | string = '/api/stats/sse'
): {
  plugins: Readable<Map<string, Plugin>>;
  systemStats: Readable<SystemStats>;
  traces: Readable<Trace[]>;
} => {
  let source = new EventSource(url);

  let plugins: Writable<Map<string, Plugin>> = writable(new Map());

  let systemStats: Writable<SystemStats> = writable({
    cpuUsage: 0,
    memory: 0,
    virtualMemory: 0,
    startTime: 0,
  });

  let traces: Writable<Trace[]> = writable([]);

  source.addEventListener('plugin', (event) => {
    const data: Plugin = JSON.parse(event.data);
    plugins.update((plugins) => plugins.set(data.id, data));
  });

  source.addEventListener('system', (event) => {
    const data: SystemStats = JSON.parse(event.data);
    systemStats.set(data);
  });

  source.addEventListener('trace', (event) => {
    const data: Trace = JSON.parse(event.data);
    traces.update((ts) => ts.concat([data]));
  });

  return { plugins, systemStats, traces };
};

export const { plugins, systemStats, traces } = makeWritables();
