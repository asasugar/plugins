use super::Preset;

pub fn get_react_preset() -> Preset {
  return Preset {
    form: "react".to_string(),
    imports: vec![
      "useState".to_string(),
      "useCallback".to_string(),
      "useMemo".to_string(),
      "useEffect".to_string(),
      "useRef".to_string(),
      "useContext".to_string(),
      "useReducer".to_string(),
    ],
  };
}
