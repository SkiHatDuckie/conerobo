import os
import pytermgui as ptg


def load_app_yaml(filename: str) -> ptg.WidgetNamespace:
    """Finds and loads a YAML file containing information about the structure of the TUI.
    TEMPORARY: The function will currently check if the working directory is `conerobo/src`,
    or `conerobo` to find YAML files. This will be changed once the project structure of ConeRobo
    is finalized."""
    with ptg.YamlLoader() as loader:
        path = filename
        if "src" in os.listdir(os.getcwd()):
            path = "src/" + filename

        return loader.load(open(path, "r", encoding="utf-8"))


if __name__ == "__main__":
    app = load_app_yaml("app.yml")

    with ptg.WindowManager() as manager:
        manager.add(app.MainWindow.center())

        manager.run()