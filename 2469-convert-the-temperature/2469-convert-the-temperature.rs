impl Solution {
    pub fn convert_temperature(celsius: f64) -> Vec<f64> {
        let Kelvin = celsius + 273.15;
        let Fahrenheit = celsius * 1.80 + 32.00;
        
        return vec![Kelvin, Fahrenheit];
    }
}