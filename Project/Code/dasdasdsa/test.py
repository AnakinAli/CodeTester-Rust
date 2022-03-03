import sys

"""
TEMPLATE EVERY CODE SHOULD HAVE
"""


def read_input_from_file(file_location):
    with open(file_location, 'r') as f:
        return f.readlines()


def main(file_location):
    """
    Write Code HERE
    :param file_location:
    :return:
    """
    inputs = read_input_from_file(file_location)
    print(inputs[0])


if __name__ == "__main__":
    main(sys.argv[1])
