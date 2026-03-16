import type { PageLoad } from './$types';

type SystemStats = {
  cpuUsage: number;
  memory: number;
  virtualMemory: number;
  startTime: number;
};

export const load: PageLoad = async ({ fetch, parent }) => {
  await parent();

  const response = await fetch('/api/stats/system');
  let data: SystemStats = await response.json();

  return {
    ...data,
  };
};
