<?php

declare(strict_types=1);

namespace App\Http\Controllers;

use App\Http\Requests\StoreProjectRequest;
use App\Http\Resources\ProjectResource;
use App\Models\Project;
use Illuminate\Http\JsonResponse;
use Illuminate\Http\Resources\Json\AnonymousResourceCollection;

final class ProjectController
{
    public function index(): AnonymousResourceCollection
    {
        return ProjectResource::collection(Project::query()->orderBy('id')->get());
    }

    public function store(StoreProjectRequest $request): JsonResponse
    {
        $project = Project::create($request->validated());
        return (new ProjectResource($project))->response()->setStatusCode(201);
    }

    public function show(Project $project): ProjectResource
    {
        return new ProjectResource($project);
    }
}
