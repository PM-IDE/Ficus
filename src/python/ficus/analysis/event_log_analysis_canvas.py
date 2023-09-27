from typing import Optional

from IPython.core.display_functions import display
from ipycanvas import Canvas, hold_canvas

from ficus.analysis.event_log_analysis import ColoredRectangle


def draw_colors_event_log_canvas(log: list[list[ColoredRectangle]],
                                 title: Optional[str] = None,
                                 plot_legend: bool = False,
                                 width_scale: float = 1,
                                 height_scale: float = 1,
                                 save_path: Optional[str] = None):
    axes_margin = 15
    axes_width = 1
    axes_padding = 5

    overall_delta = axes_margin + axes_width + axes_padding

    max_width = max(map(len, log))

    title_height = 20 if title is not None else 0
    canvas_height = len(log) * height_scale + overall_delta + title_height
    canvas = Canvas(width=max_width * width_scale + overall_delta, height=canvas_height)

    if title_height is not None:
        canvas.font = '10px'
        canvas.fill_text(title, canvas.width / 2, title_height / 2)

    canvas.stroke_style = "#000000"
    canvas.stroke_line(axes_margin, title_height, axes_margin, canvas.height - axes_margin)
    canvas.stroke_line(axes_margin, canvas.height - axes_margin, canvas.width, canvas.height - axes_margin)

    canvas.font = '10px'
    canvas.fill_text(str(len(log)), 0, 10 + title_height)
    canvas.fill_text(str(max_width), canvas.width - 2 * axes_margin, canvas.height)

    if save_path is not None:
        canvas.sync_image_data = True

    def save_to_file():
        if save_path is not None:
            canvas.to_file(save_path)

    if save_path is not None:
        canvas.observe(save_to_file, 'image_data')

    with hold_canvas():
        current_y = title_height

        for trace in log:
            current_x = overall_delta
            for rect in trace:
                canvas.fill_style = rect.color.to_hex()

                rect_width = rect.length * width_scale
                rect_height = height_scale
                canvas.fill_rect(current_x, current_y, rect_width, rect_height)
                current_x += rect_width

            current_y += height_scale

    if save_path is None:
        display(canvas)
