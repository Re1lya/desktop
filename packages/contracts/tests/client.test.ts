import assert from "node:assert/strict";
import test from "node:test";

import { createContractsClient } from "../src/client.js";
import type { ContractTransport, ContractTransportRequest } from "../src/transport.js";

/**
 * Builds a transport double that records requests and returns a fixed response.
 */
function recordingTransport<TResponse>(
  requests: ContractTransportRequest[],
  response: TResponse,
): ContractTransport {
  return {
    async send<TTransportResponse>(
      request: ContractTransportRequest,
    ): Promise<TTransportResponse> {
      requests.push(request);

      return response as unknown as TTransportResponse;
    },
  };
}

test("builds update URLs from path params and JSON bodies", async () => {
  const requests: ContractTransportRequest[] = [];
  const client = createContractsClient(
    recordingTransport(requests, {
      task: {
        id: "task-1",
        projectId: "project-1",
        title: "Ship SDK",
        status: "doing",
        worktreeId: null,
      },
    }),
  );
  const response = await client.updateTask({
    taskId: "task-1",
    projectId: "project-1",
    title: "Ship SDK",
    status: "doing",
    worktreeId: null,
  });

  assert.deepEqual(requests, [
    {
      operationName: "updateTask",
      method: "PUT",
      path: "/api/tasks/task-1",
      body: {
        projectId: "project-1",
        title: "Ship SDK",
        status: "doing",
        worktreeId: null,
      },
      headers: {
        "content-type": "application/json",
      },
    },
  ]);
  assert.deepEqual(response, {
    task: {
      id: "task-1",
      projectId: "project-1",
      title: "Ship SDK",
      status: "doing",
      worktreeId: null,
    },
  });
});

test("omits JSON bodies for path-only operations", async () => {
  const requests: ContractTransportRequest[] = [];
  const client = createContractsClient(
    recordingTransport(requests, {
      project: {
        id: "project-1",
        name: "Ora",
        rootPath: "/workspace/ora",
      },
    }),
  );

  await client.getProject({
    projectId: "project-1",
  });

  assert.deepEqual(requests, [
    {
      operationName: "getProject",
      method: "GET",
      path: "/api/projects/project-1",
      body: undefined,
      headers: {},
    },
  ]);
});
