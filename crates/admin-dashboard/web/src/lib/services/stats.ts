type SystemStatsHandler = (stats: SystemStats) => void;
type EventHandler = (event: MessageEvent) => void;

export interface StatsService {
  addSystemEventListener(func: SystemStatsHandler): void;
  removeSystemEventListener(func: SystemStatsHandler): void;
}

export type SystemStats = {
  memory: number;
  virtualMemory: number;
  cpuUsage: number;
  startTime: number;
};

export class SseStatsService implements StatsService {
  private source: EventSource;
  private handlers: Map<SystemStatsHandler, EventHandler>;

  constructor(url?: URL) {
    this.source = new EventSource(url || '/api/stats/sse');
    this.handlers = new Map();
  }

  addSystemEventListener(func: SystemStatsHandler): void {
    let handler = (event: MessageEvent) => {
      const stats: SystemStats = JSON.parse(event.data);
      func(stats);
    };

    this.handlers.set(func, handler);

    this.source.addEventListener('system', handler);
  }

  removeSystemEventListener(func: SystemStatsHandler): void {
    const handler = this.handlers.get(func);
    if (!handler) {
      return;
    }
    this.source.removeEventListener('system', handler);
    this.handlers.delete(func);
  }
}

// TODO: maybe this is not a good idea
export const statsService: StatsService = new SseStatsService();
