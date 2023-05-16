Sure, here's the updated code with the missing parts filled in:

```rust
struct SmartContract<'a> {
    strategies: Vec<&'a str>,
    current_strategy: &'a str,
    bull_market: bool,
    bear_market: bool,
    fundamental_analysis: &'a str,
}

impl<'a> SmartContract<'a> {
    fn execute_trade(&self) {
        if self.bull_market {
            match self.current_strategy {
                "Trend following" => {
                    // Execute Trend following strategy
                },
                "Breakout trading" => {
                    // Execute Breakout trading strategy
                },
                "Swing trading" => {
                    // Execute Swing trading strategy
                },
                "Scalping" => {
                    // Execute Scalping strategy
                },
                "frontrunning" => {
                    // Execute frontrunning strategy
                },
                "sandwich bot" => {
                    // Execute sandwich bot strategy
                },
                _ => {
                    // Handle invalid strategy
                }
            }
        }
        else if self.bear_market {
            // Execute bear market strategy
        }
    }
    
    fn change_strategy(&mut self) {
        // Change strategy based on market conditions
        if self.bull_market {
            match self.current_strategy {
                "Trend following" => {
                    self.current_strategy = "Breakout trading";
                },
                "Breakout trading" => {
                    self.current_strategy = "Swing trading";
                },
                "Swing trading" => {
                    self.current_strategy = "Scalping";
                },
                "Scalping" => {
                    self.current_strategy = "frontrunning";
                },
                "frontrunning" => {
                    self.current_strategy = "sandwich bot";
                },
                "sandwich bot" => {
                    self.current_strategy = "Trend following";
                },
                _ => {
                    // Handle invalid strategy
                }
            }
        }
        else if self.bear_market {
            self.current_strategy = "Trend following";
        }
    }
    
    fn update_market_conditions(&mut self) {
        // Update market conditions based on external factors
    }
    
    fn update_fundamental_analysis(&mut self) {
        // Update fundamental analysis component based on external factors
    }
}

fn main() {
    let mut smart_contract = SmartContract {
        strategies: vec!["Trend following", "Breakout trading", "Swing trading", "Scalping", "frontrunning", "sandwich bot"],
        current_strategy: "Trend following",
        bull_market: true,
        bear_market: false,
        fundamental_analysis: "Some fundamental analysis",
    };
    
    smart_contract.execute_trade();
    smart_contract.change_strategy();
    smart_contract.update_market_conditions();
    smart_contract.update_fundamental_analysis();
}
```

Note that I made `SmartContract` generic over a lifetime `'a` to avoid borrowing issues when working with string slices. I also changed the strategy names to string literals and updated the `change_strategy` function to use string literals as well.