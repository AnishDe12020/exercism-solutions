// Package weather tells you the weather.
package weather

// CurrentCondition tells you the current weather condition.
var CurrentCondition string

// CurrentLocation tells you the current location.
var CurrentLocation string

// Forecast forecasts the weather.
func Forecast(city, condition string) string {
	CurrentLocation, CurrentCondition = city, condition
	return CurrentLocation + " - current weather condition: " + CurrentCondition
}
