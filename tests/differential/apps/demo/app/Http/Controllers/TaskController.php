<?php

declare(strict_types=1);

namespace App\Http\Controllers;

use App\Data\TaskChange;
use App\Http\Requests\StoreTaskRequest;
use App\Models\Project;
use App\Models\Task;
use Illuminate\Http\JsonResponse;
use Illuminate\Http\Request;
use Spatie\LaravelData\Optional;

final class TaskController
{
    public function store(StoreTaskRequest $request, Project $project): JsonResponse
    {
        $task = $project->tasks()->create([
            'title' => $request->string('title')->toString(),
            'hours' => $request->integer('hours') ?: null,
            'done' => false,
        ]);
        return new JsonResponse(['id' => $task->id, 'title' => $task->title, 'done' => $task->done], 201);
    }

    public function update(Request $request, Task $task): JsonResponse
    {
        $change = TaskChange::from($request->all());
        if (!$change->title instanceof Optional) {
            $task->title = $change->title;
        }
        if (!$change->done instanceof Optional) {
            $task->done = $change->done;
        }
        $task->save();
        return new JsonResponse(['id' => $task->id, 'title' => $task->title, 'done' => $task->done]);
    }
}
