import type { PageLoad } from './$types';

type PluginStats = {
  id: string;
  endpoint: string;
  name: string;
  version: string;
  description: string;
};

export const load: PageLoad = async ({ parent, fetch }) => {
  await parent();

  const response = await fetch('/api/stats/plugins');
  const plugins: PluginStats[] = await response.json();

  return { plugins };
};
