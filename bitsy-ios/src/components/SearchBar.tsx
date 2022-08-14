import React from 'react';
import {Searchbar} from 'react-native-paper';
import Ionicons from 'react-native-vector-icons/Ionicons';
import {color} from '../const';

interface SearchbarProps {
  onChangeText: any;
  query: string;
}

const SearchBar = ({onChangeText, query}: SearchbarProps) => {
  return (
    <Searchbar
      placeholder="Search"
      onChangeText={onChangeText}
      value={query}
      icon={() => <Ionicons name="ios-search-outline" color={color.light_grey} size={20} />}
      clearIcon={() => <Ionicons name="ios-close-outline" color={color.light_grey} size={20} />}
    />
  );
};

export default SearchBar;
