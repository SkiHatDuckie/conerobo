from scenebase import SceneBase


class MainScene(SceneBase):
    def __init__(self):
        '''TODO: Document!'''

        SceneBase.__init__(self)      

    def processInput(self, events, pressed_keys, pressed_mouse):
        pass

    def update(self):
        pass

    def render(self, screen):
        screen.fill((255, 255, 255))
