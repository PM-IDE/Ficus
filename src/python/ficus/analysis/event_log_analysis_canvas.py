from typing import Optional

from IPython.core.display_functions import display
from ipycanvas import Canvas, hold_canvas

from ficus.analysis.event_log_analysis import ColoredRectangle

legend_rect_width = 40
legend_rect_height = 20
x_delta = 10


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

    before_height = canvas_height
    names_to_colors = None
    if plot_legend:
        names_to_colors = dict()

        for trace in log:
            for rect in trace:
                names_to_colors[rect.name] = rect.color.to_hex()

        canvas_height += len(names_to_colors) * 20

    canvas = Canvas(width=max_width * width_scale + overall_delta, height=canvas_height)

    if title_height is not None:
        canvas.font = '10px'
        canvas.fill_text(title, canvas.width / 2, title_height / 2)

    canvas.stroke_style = 'black'
    canvas.stroke_line(axes_margin, title_height, axes_margin, before_height - axes_margin)
    canvas.stroke_line(axes_margin, before_height - axes_margin, canvas.width, before_height - axes_margin)

    canvas.font = '10px'
    canvas.fill_text(str(len(log)), 0, 10 + title_height)
    canvas.fill_text(str(max_width), canvas.width - 2 * axes_margin, before_height)

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
                hex_color = rect.color.to_hex()
                canvas.fill_style = hex_color

                rect_width = rect.length * width_scale
                rect_height = height_scale
                canvas.fill_rect(current_x, current_y, rect_width, rect_height)
                current_x += rect_width

            current_y += height_scale

    if names_to_colors is not None:
        index = 0
        current_x = canvas.width / 3

        for name, color in names_to_colors.items():
            canvas.fill_style = color
            canvas.fill_rect(current_x, before_height + legend_rect_height * index, legend_rect_width,
                             legend_rect_height)

            canvas.fill_style = 'black'
            canvas.fill_text(name,
                             current_x + legend_rect_width + x_delta,
                             before_height + legend_rect_height * index + legend_rect_height / 1.4)

            index += 1

    if save_path is None:
        display(canvas)
