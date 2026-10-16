<?php

declare(strict_types=1);

namespace App;

use Illuminate\Database\Eloquent\Model;

final class Role extends Model
{
}

// A migration writes rows through an alias, which may name any table (P2R-410).
function admin(): ?Role
{
    return Role::find(1);
}
