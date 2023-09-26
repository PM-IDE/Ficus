from typing import Optional

from IPython.core.display_functions import display
from ipycanvas import Canvas, hold_canvas

from ficus.analysis.event_log_analysis import ColoredRectangle


def draw_colors_event_log_canvas(log: list[list[ColoredRectangle]],
                                 width_scale: int = 1,
                                 height_scale: int = 1,
                                 save_path: Optional[str] = None):
    max_width = max(map(len, log))
    canvas = Canvas(width=max_width * width_scale, height=len(log) * height_scale)
    if save_path is not None:
        canvas.sync_image_data = True

    def save_to_file():
        if save_path is not None:
            canvas.to_file(save_path)

    if save_path is not None:
        canvas.observe(save_to_file, 'image_data')

    with hold_canvas():
        current_y = 0

        for trace in log:
            current_x = 0
            for rect in trace:
                canvas.fill_style = rect.color.to_hex()

                rect_width = rect.length * width_scale
                rect_height = height_scale
                canvas.fill_rect(current_x, current_y, rect_width, rect_height)
                current_x += rect_width

            current_y += height_scale

    if save_path is None:
        display(canvas)
