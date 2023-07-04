from .contexts.part_results import PipelinePartResult
from ..pipelines.pipelines import InternalPipelinePart


class InternalDrawingPipelinePart(InternalPipelinePart):
    def __init__(self,
                 title: str = None,
                 plot_legend: bool = False,
                 height_scale: int = 1,
                 width_scale: int = 1,
                 save_path: str = None):
        self.title = title
        self.plot_legend = plot_legend
        self.height_scale = height_scale
        self.save_path = save_path
        self.width_scale = width_scale

    def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
        raise NotImplementedError()
