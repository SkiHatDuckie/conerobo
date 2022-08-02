class SceneBase:
    def __init__(self):
        '''Interface for creating new game scenes.'''
        self.next = self

    def process_input(self, events, pressed_keys, pressed_mouse):
        print("oh noes!, you didn't override this in the child class")

    def update(self):
        print("oh noes!, you didn't override this in the child class")

    def render(self, screen):
        print("oh noes!, you didn't override this in the child class")

    def switchToScene(self, next_scene):
        self.next = next_scene

    def terminate(self):
        self.switchToScene(None)
        print("app terminated")
