package cars

// CalculateWorkingCarsPerHour calculates how many working cars are
// produced by the assembly line every hour
func CalculateWorkingCarsPerHour(productionRate int, successRate float64) float64 {
	successRateInDecimal := successRate / float64(100)
	return float64(productionRate) * successRateInDecimal
}

// CalculateWorkingCarsPerMinute calculates how many working cars are
// produced by the assembly line every minute
func CalculateWorkingCarsPerMinute(productionRate int, successRate float64) int {
	numberOfCarsPerMinute := productionRate / 60
	return int(CalculateWorkingCarsPerHour(numberOfCarsPerMinute, successRate))
}

// CalculateCost works out the cost of producing the given number of cars
func CalculateCost(carsCount int) uint {
	numOfGroups := carsCount / 10
	numOfExtraCars := carsCount % 10
	costOfManufacturingGroups := numOfGroups * 95000
	costOfManufacturingExtraCars := numOfExtraCars * 10000
	return uint(costOfManufacturingGroups + costOfManufacturingExtraCars)
}
