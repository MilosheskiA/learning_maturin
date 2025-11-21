from typing import TypedDict

class CarDataDict(TypedDict):
    year: int
    brand: str
    num_doors: int
    model: str
    hp: float
    milage: float

def calc_time_rust() -> None:
    """
    Replicates the behavior of the python calc_time function.
    
    This function creates a vector with 100,000,000 integers and prints the length.
    """
    ...


'''
pub struct CarDataFromPython {
    pub year: u32,
    pub brand: String,
    pub num_doors: u8,
    pub model: String,
    pub hp: f32,
    pub milage: f64,
}
'''



def car_data(car_info: CarDataDict) -> str:
    """
    Replicates the behavior of the python car_data function.
    
    This function creates a vector with car brand names and prints the length.

    Args:
        car_info (CarDataDict): The car data received from Python.

    """
    ...