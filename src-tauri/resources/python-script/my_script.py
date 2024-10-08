import pydicom

def printds():
  ds = pydicom.dcmread('D:\\irt\\rustpython-demo\\test.dcm')
  return json.dumps(ds.to_json_dict())

# my_script.py
def greet(name):
    return f"Hello, {name}!"

def add(a, b):
    return a + b

def multiply(a, b):
    return a * b
