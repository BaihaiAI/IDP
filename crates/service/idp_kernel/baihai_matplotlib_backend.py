from matplotlib.backends.backend_agg import FigureCanvasAgg
from matplotlib.backends import backend_agg
from kernel_helper import display_publish_matplotlib_figures
from matplotlib.figure import Figure

FigureCanvas = FigureCanvasAgg

# List[Figure] 
# figs_need_to_flush = []

# copy from source matplotlib_inline/backend_inline.py
def new_figure_manager(num, *args, FigureClass=Figure, **kwargs):
    """
    Return a new figure manager for a new figure instance.

    This function is part of the API expected by Matplotlib backends.
    """
    return new_figure_manager_given_figure(num, FigureClass(*args, **kwargs))


def new_figure_manager_given_figure(num, figure):
    """
    Return a new figure manager for a given figure instance.

    This function is part of the API expected by Matplotlib backends.
    """
    # would call show implicit
    manager = backend_agg.new_figure_manager_given_figure(num, figure) # type: ignore

    # if not hasattr(figure, 'show'):
        # figure.show = lambda *args: display_publish_matplotlib_figures([figure])
    # figs_need_to_flush.append(figure)

    # if matplotlib.interactive(False):
    # print(f"matplotlib.is_interactive = {matplotlib.is_interactive(), matplotlib.rcParams['interactive']}")

    # show._draw_called = True
    return manager

# https://stackoverflow.com/questions/58153024/matplotlib-how-to-create-original-backend
def show(close=None, block=None) -> None:
    """
    This function is part of the API expected by Matplotlib backends.
    """
    from matplotlib._pylab_helpers import Gcf
    # convert List[matplotlib.backend_bases.FigureManagerBase] to graphic_obj.data
    display_publish_matplotlib_figures([fm.canvas.figure for fm in Gcf.get_all_fig_managers()])

# This flag is part of the API expected by Matplotlib backends.
# This flag will be reset by matplotlib.pyplot.draw_if_interactive when called
# show._draw_called = False

# not matplotlib but used in kernel after_run to flush all plot/show in exec part code
# def flush_figures():
#     # if code has plt.show()?
#     # if not show._draw_called:
#     #     return
#     if not figs_need_to_flush:
#         return

#     # deepcopy maybe stuck at infinite loop
#     # figs = deepcopy(figs_need_to_flush)
#     # figs_need_to_flush.clear()

#     figs = []
#     while figs_need_to_flush:
#         figs.append(figs_need_to_flush.pop(0))
#     return cvt_figs_to_graphic_obj(figs)
