from typing import List, Union, Tuple
import tokenize
import re
import ast
import sys

'''
alternative to split code, use tokenize: https://stackoverflow.com/a/39879026/12750738
return (
    previous_stmt_end_lineno
    previous_stmt_end_col_offset
    last_expr_lineno
    last_expr_col_offset
    last_expr_end_lineno
    last_expr_end_col_offset
)

-> Tuple[int, int, int, int, int, int]
'''
def func_ast_parse(code: str):
    if sys.version_info.minor <= 7:
        return func_ast_parse_py37(code)
    stmts = ast.parse(code, mode="exec").body
    if not stmts:
        # ast.get_source_segment
        return -1, -1, -1, -1, -1, -1
    last_stmt = stmts[-1]
    # if sys.version_info.minor <= 7:
    #     last_stmt.end_lineno = -1
    #     last_stmt.end_col_offset = -1

    # if only one stmt and stmt is class/function define
    if not isinstance(last_stmt, ast.Expr):
        return last_stmt.end_lineno, last_stmt.end_col_offset, -1, -1, -1, -1
    # last_stmt is expr
    if len(stmts) >= 2:
        previous_stmt = stmts[-2]
        # if sys.version_info.minor <= 7:
        #     previous_stmt.end_lineno = -1
        #     previous_stmt.end_col_offset = -1
        return (
            previous_stmt.end_lineno,
            previous_stmt.end_col_offset,
            last_stmt.lineno,
            last_stmt.col_offset,
            last_stmt.end_lineno,
            last_stmt.end_col_offset,
        )
    else:
        return (-1, -1, last_stmt.end_lineno, last_stmt.end_col_offset, -1, -1)


def func_ast_parse_py37(code: str):
    import io
    stmts = []
    cur_stmt_start = (0, 0)
    for token in tokenize.tokenize(io.BytesIO(code.encode()).readline):
        if token.type == tokenize.NEWLINE:
            stmts.append((cur_stmt_start, token.end))
            cur_stmt_start = (token.end[0]+1, 0)
    if not stmts:
        return -1, -1, -1, -1, -1, -1
    last_stmt = stmts[-1]
    # if only one stmt and stmt is class/function define
    ast_stmts = ast.parse(code, mode="exec").body
    if not isinstance(ast_stmts[-1], ast.Expr):
        return last_stmt[1][0], last_stmt[1][1], -1, -1, -1, -1
    # ugly alternative 1
    # lines = code.splitlines(keepends=True)
    # last_stmt_code = lines[last_stmt[0][0]][last_stmt[0][1]:]
    # for line in range(last_stmt[0][0] + 1, last_stmt[1][0] - 1):
    #     last_stmt += lines[line]
    #     # last_stmt += '\n'
    # last_stmt_code += lines[last_stmt[1][0]][last_stmt[1][1]:]

    # ugly alternative 2
    # if last_stmt[1][0] - last_stmt[0][0] <= 1 and any(["def" in code, "class" in code]):
    #     return last_stmt[1][0], last_stmt[1][1], -1, -1, -1, -1
    if len(stmts) >= 2:
        previous_stmt = stmts[-2]
        return (
            previous_stmt[1][0],
            previous_stmt[1][1],
            last_stmt[0][0],
            last_stmt[0][1],
            last_stmt[1][0],
            last_stmt[1][1],
        )
    else:
        return (-1, -1, last_stmt[1][0], last_stmt[1][1], -1, -1)


def _execute(cmd):
    import subprocess

    popen = subprocess.Popen(
        cmd, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, universal_newlines=True
    )
    assert popen.stdout is not None
    for stdout_line in iter(popen.stdout.readline, ""):
        yield stdout_line
    popen.stdout.close()
    exit_code = popen.wait()
    if exit_code != 0:
        raise Exception("Failed to execute command: %s, return %s" % (cmd, exit_code))


def fake_shell(cmd):
    for line in _execute(cmd):
        print(line, end="")


# Why IdpInteractiveShell not inherit IPython.core.interactiveshell.InteractiveShell
# Idp not require user install ipython
class IdpInteractiveShell:
    def __init__(self) -> None:
        # branch 7.34.0 IPython/core/display.py:311
        self.display_formatter = DisplayFormatter()
        self.display_pub = DisplayPub()

    def system(self, cmd: str):
        # system can't get stdout so we use subprocess
        # os.system(args)
        fake_shell(cmd.split())

    def getoutput(self, cmd: str) -> str:
        import subprocess

        """
        refs: IPython/core/tests/test_inputtransformer2.py:64
        """
        stdout: bytes = subprocess.check_output(cmd, shell=True)
        return bytes.decode(stdout, "utf-8")

    def run_line_magic(self, *args):
        """
        refs: IPython/core/magic.py:359
        """
        # magic_commands = {
        #     "matplotlib": lambda *args: print(*args)
        # }
        magic = args[0]
        # FIXME: Ipy.system() takes 2 positional arguments but 3 were given
        # if magic in {"ls", "pwd"}:
        #     return Ipy.system(self, *args)
        if magic not in {"pylab", "matplotlib", "lsmagic", "load"}:
            print(f"unsupported magic command {args}")

    def run_cell_magic(self, *args):
        """
        e.g. %%javascript
        """
        pass

    def register_post_execute(self, func):
        pass

class DisplayFormatter:
    # return format_dict, metadata_dict
    def format(self, obj, include=None, exclude=None):
        # object handled itself, don't proceed
        return {"__display_formatter": obj}, {}

# IPython/core/displaypub.py
class DisplayPub:
    def publish(self, data, metadata=None, **kwargs):
        if isinstance(data, dict) and "__display_formatter" in data:
            data = data["__display_formatter"]
        
        output = {}
        try:
            from matplotlib.figure import Figure
            import io
            import base64
            if isinstance(data, Figure):
                fig = data
                buf = io.BytesIO()
                if hasattr(fig, "_boxout"):
                    fig.savefig(buf, bbox_inches="tight")
                else:
                    fig.savefig(buf)
                # fig.savefig(buf, format='png')
                buf.seek(0)
                base64_output = base64.b64encode(buf.read())
                buf.close()
                png_data = base64_output.decode("utf-8")
                # fig._repr_png_ = lambda : png_data
                output["image/png"] = png_data
        except ModuleNotFoundError:
            pass
        for mime in [
            ("_repr_png_", "image/png"),
            ("_repr_jpeg_", "image/jpeg"),
            ("_repr_html_", "text/html"),
            ("_repr_markdown_", "text/markdown"),
            ("_repr_svg_", "image/svg+xml"),
            ("_repr_latex_", "text/latex"),
            ("_repr_json_", "application/json"),
            ("_repr_javascript_", "application/javascript"),
            ("_repr_pdf_", "application/pdf"),
        ]:
            attr, mime_key = mime[0], mime[1]
            if hasattr(data, attr):
                mime_value = getattr(data, attr)()
                if mime_value is not None:
                    output[mime_key] = mime_value
        # if hasattr(data, "text/plain"):
        #     output["text/plain"] = getattr(data, "text/plain")
        #     print(data['text/plain'])
        if not output:
            return
        import json
        sys.stdout.publish_ipython_data(json.dumps(output)) # type: ignore

    def clear_output(self, wait: bool):
        sys.stdout.clear_cell_output() # type: ignore
        try:
            from matplotlib._pylab_helpers import Gcf
            Gcf.destroy_all()
        except ModuleNotFoundError:
            pass


import builtins
# ipython publish_display_data() would check isinstance(InteractiveShell._instance, InteractiveShell)
# so we need to cheat ipython IdpInteractiveShell is instance of InteractiveShell
def isinstance_to_cheat_ipython(instance, class_) -> bool:
    # use __instancecheck__ not work
    # InteractiveShell.__instancecheck__ = lambda self, instance: True

    # try:
    #     from IPython.core.interactiveshell import InteractiveShell
    #     if class_ == InteractiveShell:
    #         return True
    # except ModuleNotFoundError:
    #     pass
    # finally:
    #     return builtins.isinstance_origin(instance, class_)
    if builtins.isinstance_origin(instance, IdpInteractiveShell): # type: ignore
        return True
    return builtins.isinstance_origin(instance, class_) # type: ignore

def init_ipython_display():
    # try:
    #     import matplotlib, matplotlib_inline
    #     matplotlib.use("module://matplotlib_inline.backend_inline")
    # except ModuleNotFoundError:
    #     pass
    try:
        import matplotlib
        # matplotlib.interactive(True)
    except ModuleNotFoundError:
        pass
    builtins.isinstance_origin = builtins.isinstance # type: ignore
    builtins.isinstance = isinstance_to_cheat_ipython
    try:
        from IPython.core.interactiveshell import InteractiveShell
        InteractiveShell._instance = IdpInteractiveShell() # type: ignore
        # assert InteractiveShell.instance() is not None
    except ModuleNotFoundError:
        pass


def load_or_skip(path: str, enable_checkpoint: str):
    if enable_checkpoint == "false":
        return
    # try:
    import dill
    dill.load_session(path)
    # except ModuleNotFoundError:
    #     pass


def after_run(session_path: str, var_path: str, enable_checkpoint: str):
    import baihai_aid
    baihai_aid.save_vars(var_path)
    if enable_checkpoint == "true":
        import dill
        dill.dump_session(session_path)


# class GraphicObj:
#     def __init__(self):
#         self.data = []


# figs: List[matplotlib.figure.Figure]
def display_publish_matplotlib_figures(figs):
    from matplotlib._pylab_helpers import Gcf

    display_pub = DisplayPub()
    for fig in figs:
        # text_plain = fig.__repr__()
        display_pub.publish(fig)
        Gcf.destroy(fig.number)


"""
cvt_magic_code
"""

ESCAPE_SINGLES = {"!", "?", "%", ",", ";", "/"}
ESCAPE_DOUBLES = {"!!", "??"}  # %% (cell magic) is handled separately
# The escape sequences that define the syntax transformations IPython will
# apply to user input.  These can NOT be just changed here: many regular
# expressions and other parts of the code may use their hardcoded values, and
# for all intents and purposes they constitute the 'IPython syntax', so they
# should be considered fixed.

ESC_SHELL = "!"  # Send line to underlying system shell
ESC_SH_CAP = "!!"  # Send line to system shell and capture output
ESC_HELP = "?"  # Find information about object
ESC_HELP2 = "??"  # Find extra-detailed information about object
ESC_MAGIC = "%"  # Call magic function
ESC_MAGIC2 = "%%"  # Call cell-magic function
ESC_QUOTE = ","  # Split args on whitespace, quote each as string and call
ESC_QUOTE2 = ";"  # Quote all args as a single string, call
ESC_PAREN = "/"  # Call first argument with rest of line as arguments


def _make_help_call(target, esc, next_input=None):
    """Prepares a pinfo(2)/psearch call from a target name and the escape
    (i.e. ? or ??)"""
    method = "pinfo2" if esc == "??" else "psearch" if "*" in target else "pinfo"
    arg = " ".join([method, target])
    # Prepare arguments for get_ipython().run_line_magic(magic_name, magic_args)
    t_magic_name, _, t_magic_arg_s = arg.partition(" ")
    t_magic_name = t_magic_name.lstrip(ESC_MAGIC)
    if next_input is None:
        return "get_ipython().run_line_magic(%r, %r)" % (t_magic_name, t_magic_arg_s)
    else:
        return (
            "get_ipython().set_next_input(%r);get_ipython().run_line_magic(%r, %r)"
            % (next_input, t_magic_name, t_magic_arg_s)
        )


def _tr_help(content):
    """Translate lines escaped with: ?

    A naked help line should fire the intro help screen (shell.show_usage())
    """
    if not content:
        return "get_ipython().show_usage()"

    return _make_help_call(content, "?")


def _tr_help2(content):
    """Translate lines escaped with: ??

    A naked help line should fire the intro help screen (shell.show_usage())
    """
    if not content:
        return "get_ipython().show_usage()"

    return _make_help_call(content, "??")


def _tr_magic(content):
    "Translate lines escaped with a percent sign: %"
    name, _, args = content.partition(" ")
    return "get_ipython().run_line_magic(%r, %r)" % (name, args)


def _tr_quote(content):
    "Translate lines escaped with a comma: ,"
    name, _, args = content.partition(" ")
    return '%s("%s")' % (name, '", "'.join(args.split()))


def _tr_quote2(content):
    "Translate lines escaped with a semicolon: ;"
    name, _, args = content.partition(" ")
    return '%s("%s")' % (name, args)


def _tr_paren(content):
    "Translate lines escaped with a slash: /"
    name, _, args = content.partition(" ")
    return "%s(%s)" % (name, ", ".join(args.split()))


tr = {
    ESC_SHELL: "get_ipython().system({!r})".format,
    ESC_SH_CAP: "get_ipython().getoutput({!r})".format,
    ESC_HELP: _tr_help,
    ESC_HELP2: _tr_help2,
    ESC_MAGIC: _tr_magic,
    ESC_QUOTE: _tr_quote,
    ESC_QUOTE2: _tr_quote2,
    ESC_PAREN: _tr_paren,
}

_help_end_re = re.compile(
    r"""(%{0,2}
                              (?!\d)[\w*]+            # Variable name
                              (\.(?!\d)[\w*]+)*       # .etc.etc
                              )
                              (\?\??)$                # ? or ??
                              """,
    re.VERBOSE,
)


def _find_assign_op(token_line) -> Union[int, None]:
    """Get the index of the first assignment in the line ('=' not inside brackets)

    Note: We don't try to support multiple special assignment (a = b = %foo)
    """
    paren_level = 0
    for i, ti in enumerate(token_line):
        s = ti.string
        if s == "=" and paren_level == 0:
            return i
        if s in {"(", "[", "{"}:
            paren_level += 1
        elif s in {")", "]", "}"}:
            if paren_level > 0:
                paren_level -= 1


def find_end_of_continued_line(lines, start_line: int):
    """Find the last line of a line explicitly extended using backslashes.

    Uses 0-indexed line numbers.
    """
    end_line = start_line
    while lines[end_line].endswith("\\\n"):
        end_line += 1
        if end_line >= len(lines):
            break
    return end_line


def assemble_continued_line(lines, start: Tuple[int, int], end_line: int):
    r"""Assemble a single line from multiple continued line pieces

    Continued lines are lines ending in ``\``, and the line following the last
    ``\`` in the block.

    For example, this code continues over multiple lines::

        if (assign_ix is not None) \
             and (len(line) >= assign_ix + 2) \
             and (line[assign_ix+1].string == '%') \
             and (line[assign_ix+2].type == tokenize.NAME):

    This statement contains four continued line pieces.
    Assembling these pieces into a single line would give::

        if (assign_ix is not None) and (len(line) >= assign_ix + 2) and (line[...

    This uses 0-indexed line numbers. *start* is (lineno, colno).

    Used to allow ``%magic`` and ``!system`` commands to be continued over
    multiple lines.
    """
    parts = [lines[start[0]][start[1] :]] + lines[start[0] + 1 : end_line + 1]
    return " ".join(
        [p.rstrip()[:-1] for p in parts[:-1]]  # Strip backslash+newline
        + [parts[-1].rstrip()]
    )  # Strip newline from last line


class PromptStripper:
    """Remove matching input prompts from a block of input.

    Parameters
    ----------
    prompt_re : regular expression
        A regular expression matching any input prompt (including continuation,
        e.g. ``...``)
    initial_re : regular expression, optional
        A regular expression matching only the initial prompt, but not continuation.
        If no initial expression is given, prompt_re will be used everywhere.
        Used mainly for plain Python prompts (``>>>``), where the continuation prompt
        ``...`` is a valid Python expression in Python 3, so shouldn't be stripped.

    Notes
    -----

    If initial_re and prompt_re differ,
    only initial_re will be tested against the first line.
    If any prompt is found on the first two lines,
    prompts will be stripped from the rest of the block.
    """

    def __init__(self, prompt_re, initial_re=None):
        self.prompt_re = prompt_re
        self.initial_re = initial_re or prompt_re

    def _strip(self, lines):
        return [self.prompt_re.sub("", l, count=1) for l in lines]

    def __call__(self, lines):
        if not lines:
            return lines
        if self.initial_re.match(lines[0]) or (
            len(lines) > 1 and self.prompt_re.match(lines[1])
        ):
            return self._strip(lines)
        return lines


class TokenTransformBase:
    """Base class for transformations which examine tokens.

    Special syntax should not be transformed when it occurs inside strings or
    comments. This is hard to reliably avoid with regexes. The solution is to
    tokenise the code as Python, and recognise the special syntax in the tokens.

    IPython's special syntax is not valid Python syntax, so tokenising may go
    wrong after the special syntax starts. These classes therefore find and
    transform *one* instance of special syntax at a time into regular Python
    syntax. After each transformation, tokens are regenerated to find the next
    piece of special syntax.

    Subclasses need to implement one class method (find)
    and one regular method (transform).

    The priority attribute can select which transformation to apply if multiple
    transformers match in the same place. Lower numbers have higher priority.
    This allows "%magic?" to be turned into a help call rather than a magic call.
    """

    # Lower numbers -> higher priority (for matches in the same location)
    priority = 10

    def sortby(self):
        return self.start_line, self.start_col, self.priority

    def __init__(self, start):
        self.start_line = start[0] - 1  # Shift from 1-index to 0-index
        self.start_col = start[1]

    @classmethod
    def find(cls, tokens_by_line):
        """Find one instance of special syntax in the provided tokens.

        Tokens are grouped into logical lines for convenience,
        so it is easy to e.g. look at the first token of each line.
        *tokens_by_line* is a list of lists of tokenize.TokenInfo objects.

        This should return an instance of its class, pointing to the start
        position it has found, or None if it found no match.
        """
        raise NotImplementedError

    def transform(self, lines: List[str]):
        """Transform one instance of special syntax found by ``find()``

        Takes a list of strings representing physical lines,
        returns a similar list of transformed lines.
        """
        raise NotImplementedError


class MagicAssign(TokenTransformBase):
    """Transformer for assignments from magics (a = %foo)"""

    @classmethod
    def find(cls, tokens_by_line):
        """Find the first magic assignment (a = %foo) in the cell."""
        for line in tokens_by_line:
            assign_ix = _find_assign_op(line)
            if (
                (assign_ix is not None)
                and (len(line) >= assign_ix + 2)
                and (line[assign_ix + 1].string == "%")
                and (line[assign_ix + 2].type == tokenize.NAME)
            ):
                return cls(line[assign_ix + 1].start)

    def transform(self, lines: List[str]):
        """Transform a magic assignment found by the ``find()`` classmethod."""
        start_line, start_col = self.start_line, self.start_col
        lhs = lines[start_line][:start_col]
        end_line = find_end_of_continued_line(lines, start_line)
        rhs = assemble_continued_line(lines, (start_line, start_col), end_line)
        assert rhs.startswith("%"), rhs
        magic_name, _, args = rhs[1:].partition(" ")

        lines_before = lines[:start_line]
        call = "get_ipython().run_line_magic({!r}, {!r})".format(magic_name, args)
        new_line = lhs + call + "\n"
        lines_after = lines[end_line + 1 :]

        return lines_before + [new_line] + lines_after


class SystemAssign(TokenTransformBase):
    """Transformer for assignments from system commands (a = !foo)"""

    @classmethod
    def find(cls, tokens_by_line):
        """Find the first system assignment (a = !foo) in the cell."""
        for line in tokens_by_line:
            assign_ix = _find_assign_op(line)
            if (
                (assign_ix is not None)
                and not line[assign_ix].line.strip().startswith("=")
                and (len(line) >= assign_ix + 2)
                and (line[assign_ix + 1].type == tokenize.ERRORTOKEN)
            ):
                ix = assign_ix + 1

                while ix < len(line) and line[ix].type == tokenize.ERRORTOKEN:
                    if line[ix].string == "!":
                        return cls(line[ix].start)
                    elif not line[ix].string.isspace():
                        break
                    ix += 1

    def transform(self, lines: List[str]):
        """Transform a system assignment found by the ``find()`` classmethod."""
        start_line, start_col = self.start_line, self.start_col

        lhs = lines[start_line][:start_col]
        end_line = find_end_of_continued_line(lines, start_line)
        rhs = assemble_continued_line(lines, (start_line, start_col), end_line)
        assert rhs.startswith("!"), rhs
        cmd = rhs[1:]

        lines_before = lines[:start_line]
        call = "get_ipython().getoutput({!r})".format(cmd)
        new_line = lhs + call + "\n"
        lines_after = lines[end_line + 1 :]

        return lines_before + [new_line] + lines_after


class EscapedCommand(TokenTransformBase):
    """Transformer for escaped commands like %foo, !foo, or /foo"""

    @classmethod
    def find(cls, tokens_by_line):
        """Find the first escaped command (%foo, !foo, etc.) in the cell."""
        for line in tokens_by_line:
            if not line:
                continue
            ix = 0
            ll = len(line)
            while ll > ix and line[ix].type in {tokenize.INDENT, tokenize.DEDENT}:
                ix += 1
            if ix >= ll:
                continue
            if line[ix].string in ESCAPE_SINGLES:
                return cls(line[ix].start)

    def transform(self, lines):
        """Transform an escaped line found by the ``find()`` classmethod."""
        start_line, start_col = self.start_line, self.start_col

        indent = lines[start_line][:start_col]
        end_line = find_end_of_continued_line(lines, start_line)
        line = assemble_continued_line(lines, (start_line, start_col), end_line)

        if len(line) > 1 and line[:2] in ESCAPE_DOUBLES:
            escape, content = line[:2], line[2:]
        else:
            escape, content = line[:1], line[1:]

        if escape in tr:
            call = tr[escape](content)
        else:
            call = ""

        lines_before = lines[:start_line]
        new_line = indent + call + "\n"
        lines_after = lines[end_line + 1 :]

        return lines_before + [new_line] + lines_after


class HelpEnd(TokenTransformBase):
    """Transformer for help syntax: obj? and obj??"""

    # This needs to be higher priority (lower number) than EscapedCommand so
    # that inspecting magics (%foo?) works.
    priority = 5

    def __init__(self, start, q_locn):
        super().__init__(start)
        self.q_line = q_locn[0] - 1  # Shift from 1-indexed to 0-indexed
        self.q_col = q_locn[1]

    @classmethod
    def find(cls, tokens_by_line):
        """Find the first help command (foo?) in the cell."""
        for line in tokens_by_line:
            # Last token is NEWLINE; look at last but one
            if len(line) > 2 and line[-2].string == "?":
                # Find the first token that's not INDENT/DEDENT
                ix = 0
                while line[ix].type in {tokenize.INDENT, tokenize.DEDENT}:
                    ix += 1
                return cls(line[ix].start, line[-2].start)

    def transform(self, lines):
        """Transform a help command found by the ``find()`` classmethod."""
        piece = "".join(lines[self.start_line : self.q_line + 1])
        indent, content = piece[: self.start_col], piece[self.start_col :]
        lines_before = lines[: self.start_line]
        lines_after = lines[self.q_line + 1 :]

        m = _help_end_re.search(content)
        if not m:
            raise SyntaxError(content)
        assert m is not None, content
        target = m.group(1)
        esc = m.group(3)

        # If we're mid-command, put it back on the next prompt for the user.
        next_input = None
        if (not lines_before) and (not lines_after) and content.strip() != m.group(0):
            next_input = content.rstrip("?\n")

        call = _make_help_call(target, esc, next_input=next_input)
        new_line = indent + call + "\n"

        return lines_before + [new_line] + lines_after


def leading_empty_lines(lines):
    """Remove leading empty lines

    If the leading lines are empty or contain only whitespace, they will be
    removed.
    """
    if not lines:
        return lines
    for i, line in enumerate(lines):
        if line and not line.isspace():
            return lines[i:]
    return lines


def leading_indent(lines):
    """Remove leading indentation.

    If the first line starts with a spaces or tabs, the same whitespace will be
    removed from each following line in the cell.
    """
    if not lines:
        return lines
    _indent_re = re.compile(r"^[ \t]+")
    m = _indent_re.match(lines[0])
    if not m:
        return lines
    space = m.group(0)
    n = len(space)
    return [l[n:] if l.startswith(space) else l for l in lines]


classic_prompt = PromptStripper(
    prompt_re=re.compile(r"^(>>>|\.\.\.)( |$)"), initial_re=re.compile(r"^>>>( |$)")
)

ipython_prompt = PromptStripper(re.compile(r"^(In \[\d+\]: |\s*\.{3,}: ?)"))


def cell_magic(lines):
    if not lines or not lines[0].startswith("%%"):
        return lines
    if re.match(r"%%\w+\?", lines[0]):
        # This case will be handled by help_end
        return lines
    magic_name, _, first_line = lines[0][2:].rstrip().partition(" ")
    body = "".join(lines[1:])
    return [
        "get_ipython().run_cell_magic(%r, %r, %r)\n" % (magic_name, first_line, body)
    ]


def make_tokens_by_line(lines: List[str]):
    """Tokenize a series of lines and group tokens by line.

    The tokens for a multiline Python string or expression are grouped as one
    line. All lines except the last lines should keep their line ending ('\\n',
    '\\r\\n') for this to properly work. Use `.splitlines(keeplineending=True)`
    for example when passing block of text to this function.

    """
    # NL tokens are used inside multiline expressions, but also after blank
    # lines or comments. This is intentional - see https://bugs.python.org/issue17061
    # We want to group the former case together but split the latter, so we
    # track parentheses level, similar to the internals of tokenize.
    NEWLINE, NL = tokenize.NEWLINE, tokenize.NL
    tokens_by_line = [[]]
    if len(lines) > 1 and not lines[0].endswith(("\n", "\r", "\r\n", "\x0b", "\x0c")):
        import warnings
        warnings.warn(
            "`make_tokens_by_line` received a list of lines which do not have lineending markers ('\\n', '\\r', '\\r\\n', '\\x0b', '\\x0c'), behavior will be unspecified"
        )
    parenlev = 0
    try:
        for token in tokenize.generate_tokens(iter(lines).__next__):
            tokens_by_line[-1].append(token)
            if (token.type == NEWLINE) or ((token.type == NL) and (parenlev <= 0)):
                tokens_by_line.append([])
            elif token.string in {"(", "[", "{"}:
                parenlev += 1
            elif token.string in {")", "]", "}"}:
                if parenlev > 0:
                    parenlev -= 1
    except tokenize.TokenError:
        # Input ended in a multiline string or expression. That's OK for us.
        pass
    if not tokens_by_line[-1]:
        tokens_by_line.pop()

    return tokens_by_line


def do_token_transforms(lines):
    TRANSFORM_LOOP_LIMIT = 500
    for _ in range(TRANSFORM_LOOP_LIMIT):
        changed, lines = do_one_token_transform(lines)
        if not changed:
            return lines

    raise RuntimeError(
        "Input transformation still changing after "
        "%d iterations. Aborting." % TRANSFORM_LOOP_LIMIT
    )


def do_one_token_transform(lines):
    """Find and run the transform earliest in the code.

    Returns (changed, lines).

    This method is called repeatedly until changed is False, indicating
    that all available transformations are complete.

    The tokens following IPython special syntax might not be valid, so
    the transformed code is retokenised every time to identify the next
    piece of special syntax. Hopefully long code cells are mostly valid
    Python, not using lots of IPython special syntax, so this shouldn't be
    a performance issue.
    """
    tokens_by_line = make_tokens_by_line(lines)
    candidates = []
    token_transformers = [
        MagicAssign,
        SystemAssign,
        EscapedCommand,
        HelpEnd,
    ]
    for transformer_cls in token_transformers:
        transformer = transformer_cls.find(tokens_by_line)
        if transformer:
            candidates.append(transformer)
    if not candidates:
        # Nothing to transform
        return False, lines
    ordered_transformers = sorted(candidates, key=TokenTransformBase.sortby)
    for transformer in ordered_transformers:
        try:
            return True, transformer.transform(lines)
        except SyntaxError:
            pass
    return False, lines


def cvt_magic_code(origin_code: str) -> str:
    """Transforms a cell of input code"""
    if not origin_code.endswith("\n"):
        origin_code += "\n"  # Ensure the cell has a trailing newline
    lines = origin_code.splitlines(keepends=True)

    transforms = [
        leading_empty_lines,
        leading_indent,
        classic_prompt,
        ipython_prompt,
        cell_magic,
    ]
    for transform in transforms:
        lines = transform(lines)

    lines = do_token_transforms(lines)
    return "".join(lines)


# def cvt_magic_code(origin_code: str) -> str:
#     import IPython.core.inputtransformer2
#     cvt = IPython.core.inputtransformer2.TransformerManager()
#     return cvt.transform_cell(origin_code)
"""
cvt_magic_code
"""


def update(python_version):
    # python_version: str python39 or python38
    import os
    import requests


    r = requests.get(
        "http://baihai.cn-bj.ufileos.com/baihai-lib/baihai_aid/VERSION.txt"
    )
    with open("VERSION.txt", "wb") as f:
        f.write(r.content)
    with open("VERSION.txt") as f:
        aim_version = f.read()
    os.remove("VERSION.txt")
    try:
        import baihai_aid
        import os

        current_version = baihai_aid.version()
        if current_version == aim_version:
            return f"The baihai_aid version {aim_version} exist."
        else:
            pass
    except ModuleNotFoundError:
        pass
    try:
        import time
        import subprocess

        def install_or_uninstall(cmd):
            result = subprocess.Popen(cmd, shell=True, stdout=subprocess.PIPE)
            i = 0  # time counter
            while True:
                assert result.stdout is not None
                outstr = result.stdout.readline().decode("gbk").strip()
                if outstr:
                    print(outstr)
                    i = 0
                else:
                    time.sleep(1)
                    i += 1
                if i >= 5:
                    break

        r = requests.get(
            r"http://baihai.cn-bj.ufileos.com/baihai-lib/baihai_aid/baihai-aid-"
            + str(aim_version)
            + ".tar.gz"
        )
        with open("baihai-aid-" + str(aim_version) + ".tar.gz", "wb") as f:
            f.write(r.content)

        if os.path.exists(r"/opt/"):
            write_path = (
                "/opt/share-site/miniconda3/envs/"
                + python_version
                + "/lib/"
                + os.listdir(
                    r"/opt/share-site/miniconda3/envs/" + python_version + "/lib"
                )[0]
                + "/site-packages"
            )
            cmd = (
                r"pip install -U baihai-aid-" + aim_version + ".tar.gz -t " + write_path
            )
        else:
            cmd = r"pip install -U baihai-aid-" + aim_version + ".tar.gz"

        install_or_uninstall(cmd)
        os.remove(r"baihai-aid-" + str(aim_version) + ".tar.gz")
        print("Success!")
    except TypeError:
        print("This edition doesn't exist.")
