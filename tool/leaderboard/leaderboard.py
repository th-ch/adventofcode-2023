from jinja2 import Environment, FileSystemLoader

templateLoader = FileSystemLoader(searchpath="./tool/leaderboard/")
templateEnv = Environment(loader=templateLoader)

template = templateEnv.get_template("leaderboard_template.html")

def generate_leaderboard(results):
    rendered = template.render(res=results)
    with open("./leaderboard/index.html","w+") as f:
        f.write(rendered)