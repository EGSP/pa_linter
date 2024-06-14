<script lang="ts">
	import IconExclamationTriangle from '$lib/icons/IconExclamationTriangle.svelte';
	import { type AnalysisResult, type RepositoryTree, type RepositoryTreeEntry } from '$lib/types';
	import { Accordion, AccordionItem, TreeView } from 'carbon-components-svelte';
	import { invoke } from '@tauri-apps/api';
	import IAnalysisResult from './analyze/IAnalysisResult.svelte';
	import IProjectArenaTree from './structure/IProjectArenaTree.svelte';
	import { SvelteComponent, onMount } from 'svelte';
	import Frame from '$lib/components/Frame.svelte';
	import CarbonRun from '$lib/icons/CarbonRun.svelte';
	import TdesignRefresh from '$lib/icons/TdesignRefresh.svelte';
	import VscodeIconsFileTypeJson from '$lib/icons/files/VscodeIconsFileTypeJson.svelte';

	import { Button } from 'carbon-components-svelte';
	import Label from '$lib/components/Label.svelte';
	import type { TreeNode } from 'carbon-components-svelte/src/TreeView/TreeView.svelte';

	import { Pane, Splitpanes } from 'svelte-splitpanes';

	let analysis_results = new Array<AnalysisResult>();
	async function analyze() {
		analysis_results = await invoke<AnalysisResult[]>('c_analyze_repositories', {
			folderPath: 'c:/Workroot/softdev/pa_linter_test'
		});

		console.log(analysis_results);
	}

	let repository_node_trees: Array<TreeNode[]> = [];
	let repository_node_trees_names: Array<string> = [];
	// added this boolean cause array lenght condition didnt work for some reason
	let tree_view_ini:boolean = false;
	async function get_project_tree() {
		tree_view_ini = false;
		repository_node_trees = [];
		repository_node_trees_names = [];
		

		console.log('get_project_tree');
		let repository_trees = await invoke<RepositoryTree[]>('c_get_project_trees');
		// console.log(repository_trees);

		for (let repository_tree of repository_trees) {
			console.log('Building tree for ' + repository_tree.repository_info.mod_identifier);
			// console.log(repository_tree.entries);
			let nodes_tree = new Array<TreeNode>();

			for (let repository_entry of repository_tree.entries) {
				build_node(repository_entry);

				function find_node_in_tree(id: string): TreeNode | null {
					for (let node of nodes_tree) {
						if (node.id == id) {
							return node;
						}

						let found = find_node_in_tree_node(node);
						if (found != null) {
							return found;
						}
					}

					function find_node_in_tree_node(node: TreeNode): TreeNode | null {
						if (node.children == null) {
							return null;
						}
						for (let child of node.children!) {
							if (child.id == id) {
								return child;
							} else {
								let found = find_node_in_tree_node(child);
								if (found != null) {
									return found;
								}
							}
						}
						return null;
					}

					return null;
				}

				function build_node(repository_entry: RepositoryTreeEntry): TreeNode {
					let same_node_in_project_tree = find_node_in_tree(repository_entry.id.toString());

					// another nodes could create that node while constructing parent tree
					if (same_node_in_project_tree != null) {
						// console.log('Node %d already in project tree', same_node_in_project_tree.id);
						return same_node_in_project_tree;
					} else {
						let text = repository_entry.path.split('\\').pop();
						let node = {
							id: repository_entry.id.toString(),
							text,
							children: repository_entry.children.length > 0 ? new Array<TreeNode>() : undefined
						};
						// console.log(repository_entry)
						// console.log(node);

						// console.log('Adding node %d to project tree', node.id);
						add_to_parent_or_root(node, repository_entry);
						return node;
					}
				}

				function add_to_parent_or_root(new_node: TreeNode, repository_entry: RepositoryTreeEntry) {
					if (repository_entry.parent == null) {
						// push root node
						// console.log('Adding root node %d to project tree root', new_node.id);
						nodes_tree.push(new_node);
					} else {
						// find parent node in project tree
						// console.log('Look for parent %d for node %d', repository_entry.parent, new_node.id);
						let parent_node_in_project_tree = find_node_in_tree(repository_entry.parent.toString());

						if (parent_node_in_project_tree != null) {
							// console.log(
							// 	'Get parent %d for node %d',
							// 	parent_node_in_project_tree?.id,
							// 	new_node.id
							// );
							// console.log(
							// 	'same parent %d for node %d, and repository node`s %d parent is %d',
							// 	parent_node_in_project_tree.id,
							// 	new_node.id,
							// 	repository_entry.id,
							// 	repository_entry.parent
							// );

							// console.log('Add node %d to parent %d', new_node.id, parent_node_in_project_tree.id);
							
							if(parent_node_in_project_tree.children == null){
								console.log('children is undefined for node %d', parent_node_in_project_tree.id);
							}
							parent_node_in_project_tree!.children!.push(new_node);
						} else {
							// console.log('Find repository_entry.parent %d in repository_tree', repository_entry.parent);
							// creating parent node in project tree
							let repository_entry_parent = repository_tree.entries.find(
								(entry) => entry.id === repository_entry.parent
							);
							// console.log(
							// 	'No parent in project tree for node %d . Building node %d',
							// 	new_node.id,
							// 	repository_entry_parent?.id
							// );
							let parent_node = build_node(repository_entry_parent!);
							parent_node.children!.push(new_node);
						}
					}
				}
			}

			console.log(nodes_tree);
			repository_node_trees.push(nodes_tree);
			repository_node_trees_names.push(repository_tree.repository_info.mod_identifier);
		}
		
		// console.log('repository_node_trees');
		// console.log(repository_node_trees);
		// console.log(repository_node_trees.length)

		tree_view_ini = true;

		function get_icon_for_file(file_name: string): SvelteComponent<any> | undefined {
			// select icon based on file extension
			let file_extension = file_name.split('.').pop();
			switch (file_extension) {
				case 'json': return ;
				// TODO: ДОБАВИТЬ РЕАЛИЗАЦИЮ ИКОНОК. НЕ СДЕЛАЛ С САМОГО НАЧАЛА, ПОТОМУ ЧТО ИКОНКИ НУЖНО ОБОРАЧИВАТЬ В КОМПОНЕНТ ПОДХОДЯЩИЙ
				// directory
				case undefined: ;
				// common file
				default: ;
			}

			return undefined;
		}
	}

	onMount(async () => {
		await get_project_tree();
		// await analyze_tree();
	});
</script>

<Frame>
	<Splitpanes>
		<Pane>
			<Frame direction={'column'}>
				<div class="action-bar">
					<Button
						on:click={get_project_tree}
						kind="secondary"
						size="small"
						icon={TdesignRefresh}
						iconDescription="Refresh project tree"
						tooltipPosition="right"
						tooltipAlignment="end"
					/>
					<Button
						on:click={analyze}
						kind="primary"
						size="small"
						icon={CarbonRun}
						iconDescription="Analyze project folder"
						tooltipPosition="right"
						tooltipAlignment="end"
					/>
				</div>

				<div class="project-tree" id="project-tree">
					{#if tree_view_ini}
						{#each repository_node_trees as nodes_tree, i}
							<Label text={repository_node_trees_names[i]} />
							<TreeView labelText="Project Tree" children={nodes_tree} style="overflow:unset" size="compact" />
						{/each}
					{:else}
						<p>No Trees</p>
					{/if}
				</div>
			</Frame>
		</Pane>
		<Pane>
			<Frame direction={'column'}>
				<Accordion align="start" size="sm">
					{#if analysis_results.length > 0}
						{#each analysis_results as result, i}
							<AccordionItem>
								<svelte:fragment slot="title">
									<div class="flex">
										<IconExclamationTriangle />
										<!-- {result.file_path} -->
										<Label text={result.file_path} />
									</div>
								</svelte:fragment>
								<IAnalysisResult tips={result.tips} />
							</AccordionItem>
						{/each}
					{:else}
						<p>No results</p>
					{/if}
				</Accordion>
			</Frame>
		</Pane>
	</Splitpanes>
</Frame>

<style>
	.project-tree {
		display: flex;
		flex-direction: column;
		text-wrap: nowrap;

		width: 100%;
		height: 100%;
	}

	.action-bar {
		display: flex;
		flex: none;
		flex-flow: row;
		width: 100%;
	}
</style>
