from matplotlib.backends.backend_agg import FigureCanvasAgg
from kernel_helper import GraphicObj

FigureCanvas = FigureCanvasAgg

def show(close=None, block=None) -> GraphicObj:
    from matplotlib._pylab_helpers import Gcf
    from kernel_helper import cvt_figs_to_graphic_obj
    fig_managers = Gcf.get_all_fig_managers()
    return cvt_figs_to_graphic_obj(map(lambda fm: fm.canvas, fig_managers))
