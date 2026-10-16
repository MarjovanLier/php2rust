<?php

declare(strict_types=1);

namespace App;

use Illuminate\Database\Eloquent\Model;

// The store's paths start with ::std, because the namespace App\Std gives the crate a module
// std beside the model.
final class Role extends Model
{
}

function admin(): ?Role
{
    return Role::find(1);
}
