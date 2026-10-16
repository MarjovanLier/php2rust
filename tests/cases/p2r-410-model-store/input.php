<?php

declare(strict_types=1);

namespace App;

use Illuminate\Database\Eloquent\Model;

final class Role extends Model
{
    protected $fillable = ['name', 'description', 'level', 'active'];

    protected $casts = ['active' => 'bool'];
}

function roleName(int $id): string
{
    $role = Role::query()->find($id);
    if ($role === null) {
        return 'none';
    }
    return $role->name;
}

function byLevel(int $level): ?Role
{
    return Role::query()->where('level', $level)->first();
}

function described(string $description): string
{
    $role = Role::query()->where('description', $description)->first();
    return $role === null ? 'none' : $role->name;
}

// The store starts empty, and each create adds a row numbered after the ones before it.
$a = Role::create(['name' => 'admin', 'description' => 'all', 'level' => 3, 'active' => true]);
$b = Role::query()->create(['level' => 5, 'name' => 'user', 'active' => false]);
echo $a->id, ' ', $b->id, ' ', $b->active ? 'on' : 'off', "\n";
echo roleName(2), ' ', roleName(9), "\n";
$found = byLevel(5);
echo $found === null ? 'none' : $found->name, "\n";
echo described('all'), ' ', described('ALL'), "\n";
$c = Role::find(1);
if ($c !== null) {
    $description = $c->description;
    if ($description !== null) {
        echo $description, "\n";
    }
}
$d = Role::query()->where('description', null)->where('level', 5)->first();
echo $d === null ? 'none' : $d->name, "\n";
$e = Role::query()->where('name', 'admin')->where('active', true)->first();
if ($e !== null) {
    echo $e->id, "\n";
}
