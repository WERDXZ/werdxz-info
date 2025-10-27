import type { Project } from "../lib/api.ts";

export function StageBadge({ stage }: { stage: Project["stage"] }) {
  const stageLabels: Record<typeof stage, string> = {
    planned: "Planned",
    wip: "WIP/MVP",
    active: "Active",
    maintained: "Maintained",
    archived: "Archived",
    shelved: "Shelved",
  };

  return <span class={`badge stage-badge stage-${stage}`}>{stageLabels[stage]}</span>;
}
