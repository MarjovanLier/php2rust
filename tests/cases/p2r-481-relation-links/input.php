<?php

declare(strict_types=1);

namespace App;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsToMany;
use Illuminate\Database\Eloquent\Relations\HasMany;
use Illuminate\Database\Eloquent\Relations\HasOne;

// Relation methods with the links of P2R-481 after the helper leave the output, as casts()
// does: Laravel runs the links only when the relation method runs, which nothing here does.
final class Role extends Model
{
    protected $fillable = ['name'];

    public $timestamps = false;

    public function permissions(): BelongsToMany
    {
        return $this->belongsToMany(Permission::class, 'role_permissions')
            ->withTimestamps()
            ->withPivot('deleted_at', "granted_by")
            ->wherePivotNull('deleted_at');
    }

    public function latestAddress(): HasOne
    {
        return $this->hasOne(Address::class)->with('city');
    }
}

final class Permission extends Model
{
    protected $fillable = ['name'];

    public $timestamps = false;

    public function roles(): BelongsToMany
    {
        return $this->belongsToMany(Role::class, 'role_permissions')->limit(1);
    }

    public function addresses(): HasMany
    {
        return $this->hasMany(Address::class)->limit(5)->with('city', 'country');
    }
}

final class Address extends Model
{
    protected $fillable = ['street'];

    public $timestamps = false;
}

$role = Role::create(['name' => 'admin']);
$permission = Permission::create(['name' => 'post']);
$found = Role::find(1);
echo $role->id, ' ', $permission->name, ' ', $found === null ? '-' : $found->name, "\n";
