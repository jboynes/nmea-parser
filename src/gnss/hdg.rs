/*
Copyright 2026 Jeremy Boynes

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

use super::*;

/// HDG - Heading, deviation and variation
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct HdgData {
    /// Magnetic sensor heading, degrees
    pub heading: Option<f64>,
    /// Magnetic deviation, degrees;
    pub deviation: Option<f64>,
    /// Magnetic variation, degrees
    pub variation: Option<f64>,
}

// -------------------------------------------------------------------------------------------------

/// xxHDG: Heading, deviation and variation
pub(crate) fn handle(sentence: &str) -> Result<ParsedMessage, ParseError> {
    let split: Vec<&str> = sentence.split(',').collect();

    let heading = pick_number_field(&split, 1)?;
    let deviation = if let Some(val) = pick_number_field::<f64>(&split, 2)? {
        let side = split.get(3).unwrap_or(&"");
        match *side {
            "E" => Some(val),
            "W" => Some(-val),
            _ => {
                return Err(format!("Invalid HDG deviation side: {}", side).into());
            }
        }
    } else {
        None
    };
    let variation = if let Some(val) = pick_number_field::<f64>(&split, 4)? {
        let side = split.get(5).unwrap_or(&"");
        match *side {
            "E" => Some(val),
            "W" => Some(-val),
            _ => {
                return Err(format!("Invalid HDG variation side: {}", side).into());
            }
        }
    } else {
        None
    };
    Ok(ParsedMessage::Hdg(HdgData {
        heading,
        deviation,
        variation,
    }))
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_hdg() {
        match NmeaParser::new().parse_sentence("$AIHDG,52.9,,,13.5,W*31") {
            Ok(ps) => match ps {
                ParsedMessage::Hdg(hdg) => {
                    assert_eq!(hdg.heading, Some(52.9));
                    assert_eq!(hdg.deviation, None);
                    assert_eq!(hdg.variation, Some(-13.5));
                }
                _ => {
                    assert!(false);
                }
            },
            Err(e) => {
                assert_eq!(e.to_string(), "OK");
            }
        }
    }
}
