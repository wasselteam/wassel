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

const makeWritables = (
  url: URL | string = '/api/stats/sse'
): {
  systemStats: Readable<SystemStats>;
  traces: Readable<Trace[]>;
} => {
  let source = new EventSource(url);

  let systemStats: Writable<SystemStats> = writable({
    cpuUsage: 0,
    memory: 0,
    virtualMemory: 0,
    startTime: 0,
  });

  let traces: Writable<Trace[]> = writable([]);

  source.addEventListener('system', (event) => {
    const data: SystemStats = JSON.parse(event.data);
    systemStats.set(data);
  });

  source.addEventListener('trace', (event) => {
    const data: Trace = JSON.parse(event.data);
    traces.update((ts) => ts.concat([data]));
  });

  return { systemStats, traces };
};

export const { systemStats, traces } = makeWritables();
