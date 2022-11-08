{
compat | doc "Nix compatibility layer. should not be used by Nickel program."
= {
  has_field_path = fun fields record =>
      %length% fields == 0 || # Because it's only used by generated code, this length will never be initialy 0. So if it's 0, it mean the end of the path.
      (
        %has_field% (%head% fields) record &&
	has_field_path (%tail% fields) record."%{%head% fields}"
      ),

  with =
    let AssertFound = fun l val => if val.found 
      then val.value
      else  %blame% l in
    fun envs field => (
    array.fold (fun current acc =>
      if !acc.found && record.has_field field current
      then { value = current."%{field}", found = true}
      else acc
  ) {value = null, found = false} envs) | AssertFound
}
}
