import math
import os
from typing import List, Union, Callable

from matplotlib import pyplot as plt, axes

from .event_log_analysis_entropy import calculate_default_entropies
from .event_log_split import split_log_by_traces
from ..grpc_pipelines.context_values import Color, ColoredRectangle
from ..log.event_log import MyEventLog
from ..log.functions import read_log_from_xes
from ..util import *


def _create_array_for_hist(log: MyEventLog) -> (List[int], int):
    unique_traces = split_log_by_traces(log)
    hist = []
    eq_class = 1
    for traces_list in unique_traces:
        for _ in range(len(traces_list)):
            hist.append(eq_class)

        eq_class += 1

    return hist, eq_class


def draw_traces_histogram(log: MyEventLog,
                          save_path: str = None,
                          title: str = None):
    hist, classes_count = _create_array_for_hist(log)
    _do_paint_hist(plt.axis(), hist, classes_count)
    plt.suptitle(title)

    if save_path is None:
        plt.show()
    else:
        plt.savefig(save_path)


def _do_paint_hist(drawer: Union[axes.Axes], hist: list[int], classes_count: int):
    drawer.hist(hist, histtype='stepfilled', bins=[i for i in range(1, classes_count + 1)])


def plot_logs_traces_histograms_for_paths(paths: List[str],
                                          save_path: str = None,
                                          title: str = None):
    logs = [read_log_from_xes(path) for path in paths]
    plot_logs_traces_histograms(logs, paths, save_path=save_path, title=title)


def plot_logs_traces_histograms(logs: List[MyEventLog],
                                logs_names: List[str],
                                save_path: str = None,
                                title: str = None):
    side_length = int(math.ceil(math.sqrt(len(logs))))
    log_index = 0

    current_figure, ax = plt.subplots(side_length, side_length, figsize=(12, 12))

    for row in range(side_length):
        for col in range(side_length):
            if log_index >= len(logs):
                break

            file_name = os.path.basename(logs_names[log_index])
            file_name_wo_ext = os.path.splitext(file_name)[0]
            ax[row, col].set_title(file_name_wo_ext, fontsize=10)

            (hist, classes_count) = _create_array_for_hist(logs[log_index])
            _do_paint_hist(ax[row, col], hist, classes_count)

            log_index += 1

    current_figure.suptitle(title)

    if save_path is not None:
        current_figure.savefig(save_path)
        plt.close(current_figure)
    else:
        current_figure.show()


class TraceDiversityLikeDiagramContext:
    def __init__(self,
                 log: MyEventLog,
                 drawer,
                 rect_width: float,
                 rect_height: float,
                 y_delta_between_traces: float,
                 height_scale: int):
        self.drawer = drawer
        self.log = log
        self.rect_width = rect_width
        self.rect_height = rect_height
        self.y_delta_between_traces = y_delta_between_traces
        self.names_to_rects = dict()
        self.height_scale = height_scale


def _draw_traces_diversity_like_diagram_internal(log: Union[MyEventLog, list[list[ColoredRectangle]]],
                                                 draw_func: Callable[[TraceDiversityLikeDiagramContext], None],
                                                 title: str = None,
                                                 save_path: str = None,
                                                 plot_legend: bool = True,
                                                 height_scale: int = 1,
                                                 width_scale: int = 1):
    rect_width = 1 * width_scale
    rect_height = 1 * height_scale
    y_delta_between_traces = 0
    current_figure, ax = plt.subplots(figsize=(20, 20))

    ctx = TraceDiversityLikeDiagramContext(log, ax, rect_width, rect_height, y_delta_between_traces, height_scale)

    draw_func(ctx)

    if plot_legend:
        ax.legend(ctx.names_to_rects.values(),
                  ctx.names_to_rects.keys(),
                  loc='upper center',
                  bbox_to_anchor=(0.5, -0.15),
                  fontsize=20)

    y_ticks_count = len(log) * height_scale + 1
    y_ticks = ['' for _ in range(y_ticks_count)]
    y_ticks[0] = '0'
    y_ticks[-1] = f'{int((len(y_ticks) - 1) / height_scale)}'
    ax.set_yticklabels(y_ticks)
    ax.set_yticks(range(y_ticks_count))

    x_ticks_count = max([len(t) for t in log]) * width_scale + 1
    x_ticks = ['' for _ in range(x_ticks_count)]
    x_ticks[0] = '0'
    x_ticks[-1] = f'{int((len(x_ticks) - 1) / width_scale)}'
    ax.set_xticklabels(x_ticks)
    ax.set_xticks(range(x_ticks_count))

    ax.tick_params(axis='y', length=0)
    ax.tick_params(axis='x', length=0)
    ax.axis('scaled')

    if title is not None:
        ax.set_xlabel(title)

    if save_path is None:
        current_figure.show()
    else:
        os.makedirs(os.path.dirname(save_path), exist_ok=True)
        current_figure.savefig(save_path, bbox_inches='tight', dpi=150)
        plt.close(current_figure)


def _draw_traces_diversity_like_diagram(log: MyEventLog,
                                        cached_colors: dict[str, str],
                                        title: str = None,
                                        save_path: str = None,
                                        plot_legend: bool = True,
                                        height_scale: int = 1,
                                        width_scale: int = 1,
                                        colors_provider: Callable[[str], str] = None):
    if colors_provider is None:
        colors = random_unique_color_provider_instance

        def generate_color(_: str):
            return colors.next()

        colors_provider = generate_color

    def draw_func(ctx: TraceDiversityLikeDiagramContext):
        current_y = 0
        names_to_colors = dict()
        for trace in log:
            current_x = 0
            for event in trace:
                name = event[concept_name]
                if name in names_to_colors:
                    color = names_to_colors[name]
                else:
                    color = cached_colors[name] if name in cached_colors else colors_provider(name)
                    cached_colors[name] = color
                    names_to_colors[name] = color

                rect = plt.Rectangle((current_x, current_y), ctx.rect_width, ctx.rect_height, fc=color)
                ctx.names_to_rects[name] = rect
                ctx.drawer.add_patch(rect)
                current_x += ctx.rect_width

            current_y += ctx.rect_height + ctx.y_delta_between_traces

    _draw_traces_diversity_like_diagram_internal(log,
                                                 draw_func,
                                                 title=title,
                                                 save_path=save_path,
                                                 plot_legend=plot_legend,
                                                 height_scale=height_scale,
                                                 width_scale=width_scale)


def draw_colors_event_log(log: list[list[ColoredRectangle]],
                          title: str = None,
                          save_path: str = None,
                          plot_legend: bool = True,
                          height_scale: int = 1,
                          width_scale: int = 1):
    def draw_func(ctx: TraceDiversityLikeDiagramContext):
        current_y = 0
        for trace in log:
            current_x = 0
            for colored_rect in trace:
                width = ctx.rect_width * colored_rect.length
                rect = plt.Rectangle((current_x, current_y), width, ctx.rect_height, fc=colored_rect.color.to_hex())
                ctx.names_to_rects[colored_rect.name] = rect
                ctx.drawer.add_patch(rect)
                current_x += width

            current_y += ctx.rect_height + ctx.y_delta_between_traces

    _draw_traces_diversity_like_diagram_internal(log,
                                                 draw_func,
                                                 title=title,
                                                 save_path=save_path,
                                                 plot_legend=plot_legend,
                                                 height_scale=height_scale,
                                                 width_scale=width_scale)


def draw_traces_diversity_diagram(log: MyEventLog,
                                  cached_colors: dict[str, str],
                                  title: str = None,
                                  save_path: str = None,
                                  plot_legend: bool = True,
                                  height_scale: int = 1,
                                  width_scale: int = 1):
    _draw_traces_diversity_like_diagram(log,
                                        cached_colors,
                                        title=title,
                                        save_path=save_path,
                                        plot_legend=plot_legend,
                                        height_scale=height_scale,
                                        width_scale=width_scale)


def draw_placements_of_concrete_events(log: MyEventLog,
                                       cached_colors: dict[str, str],
                                       events: set[str],
                                       title: str = None,
                                       save_path: str = None,
                                       height_scale: int = 1,
                                       plot_legend: bool = True):
    def predicate(event_name: str) -> bool:
        return event_name in events

    draw_placements_of_events(log, cached_colors, predicate, title, save_path, height_scale, plot_legend)


def draw_placements_of_events(log: MyEventLog,
                              cached_colors: dict[str, str],
                              events_predicate: Callable[[str], bool],
                              title: str = None,
                              save_path: str = None,
                              height_scale: int = 1,
                              plot_legend: bool = True):
    colors_provider = random_unique_color_provider_instance
    colors_provider.used_colors.add((0, 0, 0))
    default_color = '#000000'

    def generate_color(name: str) -> str:
        if events_predicate(name):
            if name in cached_colors:
                return cached_colors[name]

            cached_colors[name] = colors_provider.next()
            return cached_colors[name]

        return default_color

    _draw_traces_diversity_like_diagram(log,
                                        dict(),
                                        title=title,
                                        save_path=save_path,
                                        plot_legend=plot_legend,
                                        height_scale=height_scale,
                                        colors_provider=generate_color)


def draw_events_entropy_histogram(log: MyEventLog,
                                  title: str = None,
                                  save_path: str = None):
    entropies = calculate_default_entropies(log)

    current_x = 0
    rect_length = 1
    names_to_rects = dict()
    colors_provider = random_unique_color_provider_instance

    current_figure, ax = plt.subplots(1, 1, figsize=(20, 20))

    for (event_name, event_entropy) in entropies.items():
        color = colors_provider.next()
        rect = plt.Rectangle((current_x, 0), rect_length, event_entropy, fc=color)
        current_x += rect_length
        names_to_rects[event_name] = rect
        ax.add_patch(rect)

    keys = names_to_rects.keys()
    lst = [(key, rect, entropies[key]) for key, rect in zip(keys, names_to_rects.values())]
    lst = sorted(lst, key=lambda x: x[2], reverse=True)
    keys = [f'{key} {entropies[key]}' for key, _, _ in lst]
    rects = [rect for _, rect, _ in lst]

    ax.legend(rects, keys, loc='upper center', bbox_to_anchor=(0.5, -0.05))
    ax.axis('scaled')

    if title is not None:
        ax.set_xlabel(title)

    if save_path is None:
        current_figure.show()
    else:
        current_figure.savefig(save_path, bbox_inches='tight', dpi=150)
        plt.close(current_figure)
