import example
from unittest import TestCase

class TestExample(TestCase):

    def test(self):
        self.assertEqual(example.double(2), 4)
